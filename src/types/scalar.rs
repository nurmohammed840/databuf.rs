use crate::*;
use std::mem::size_of;

impl Encode for bool {
    #[inline]
    fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl Decode<'_> for bool {
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        u8::decode::<CONFIG>(c).map(|byte| byte != 0)
    }
}

impl Encode for char {
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        u32::from(*self).encode::<CONFIG>(c)
    }
}
impl Decode<'_> for char {
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        let num = u32::decode::<CONFIG>(c)?;
        char::from_u32(num).ok_or_else(|| format!("{num} is not a valid char").into())
    }
}

// ----------------------------------------------------------------------------------------------

impl Encode for u8 {
    #[inline]
    fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&[*self])
    }
}

impl Decode<'_> for u8 {
    #[inline]
    fn decode<const CONFIG: u8>(reader: &mut &[u8]) -> Result<Self> {
        if !reader.is_empty() {
            unsafe {
                let byte = reader.get_unchecked(0);
                *reader = reader.get_unchecked(1..);
                Ok(*byte)
            }
        } else {
            Err(Box::new(error::InsufficientBytes))
        }
    }
}

impl Encode for i8 {
    #[inline]
    fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}
impl Decode<'_> for i8 {
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        u8::decode::<CONFIG>(c).map(|byte| byte as i8)
    }
}

#[rustfmt::skip]
macro_rules! leb128_num {
    (@ubit: u16) => { u16 };
    (@ubit: u32) => { u32 };
    (@ubit: u64) => { u64 };
    (@ubit: u128) => { u128 };
    (@ubit: usize) => { usize };
    (@ubit: i16) => { u16 };
    (@ubit: i32) => { u32 };
    (@ubit: i64) => { u64 };
    (@ubit: i128) => { u128 };
    (@ubit: isize) => { usize };
    (@ubit: f32) => { u32 };
    (@ubit: f64) => { u64 };

    (@encode: u16, $this: tt) => { *$this };
    (@encode: u32, $this: tt) => { *$this };
    (@encode: u64, $this: tt) => { *$this };
    (@encode: u128, $this: tt) => { *$this };
    (@encode: usize, $this: tt) => { *$this };
    (@encode: i16, $this: tt) => { (($this << 1) ^ ($this >> 15)) as u16 };
    (@encode: i32, $this: tt) => { (($this << 1) ^ ($this >> 31)) as u32 };
    (@encode: i64, $this: tt) => { (($this << 1) ^ ($this >> 63)) as u64 };
    (@encode: i128, $this: tt) => { (($this << 1) ^ ($this >> 127)) as u128 };
    (@encode: isize, $this: tt) => { (($this << 1) ^ ($this >> isize::BITS - 1)) as usize };
    (@encode: f32, $this: tt) => { $this.to_bits() };
    (@encode: f64, $this: tt) => { $this.to_bits() };

    (@decode: u16, $num: tt) => { $num };
    (@decode: u32, $num: tt) => { $num };
    (@decode: u64, $num: tt) => { $num };
    (@decode: u128, $num: tt) => { $num };
    (@decode: usize, $num: tt) => { $num };
    (@decode: i16, $num: tt) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
    (@decode: i32, $num: tt) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
    (@decode: i64, $num: tt) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
    (@decode: i128, $num: tt) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
    (@decode: isize, $num: tt) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
    (@decode: f32, $num: tt) => { Self::from_bits($num) };
    (@decode: f64, $num: tt) => { Self::from_bits($num) };
}
#[test]
fn test_name() {
    let a = -127;
    println!("{:b}", -a);
}
// const A: u32 = 0b1111;
macro_rules! impl_data_type_for {
    [$($rty:tt)*] => ($(
        impl Encode for $rty {
            #[inline] fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
                match CONFIG & config::num::GET {
                    config::num::LE => writer.write_all(&self.to_le_bytes()),
                    config::num::BE => writer.write_all(&self.to_be_bytes()),
                    config::num::NE => writer.write_all(&self.to_ne_bytes()),
                    config::num::LEB128 => {
                        let mut num = leb128_num!(@encode: $rty, self);
                        while num > 0x7F {
                            writer.write_all(&[num as u8 | 0x80])?;
                            num >>= 7;
                        }
                        writer.write_all(&[num as u8])
                    },
                    _ => unreachable!()
                }
            }
        }
        impl Decode<'_> for $rty {
            fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
                Ok(match CONFIG & config::num::GET {
                    config::num::LE => Self::from_le_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::BE => Self::from_be_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::NE => Self::from_ne_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::LEB128 => {
                        let mut shift: u8 = 0;
                        let mut num: leb128_num!(@ubit: $rty) = 0;
                        loop {
                            let byte = u8::decode::<CONFIG>(c)?;
                            num |= ((byte & 0b0111_1111) as leb128_num!(@ubit: $rty)) << shift;
                            if byte & 0b1000_0000 == 0 {
                                break leb128_num!(@decode: $rty, num);
                            }
                            shift += 7;
                        }
                    },
                    _ => unreachable!()
                })
            }
        }
    )*);
}

impl_data_type_for!(
    u16 u32 u64 u128 usize
    i16 i32 i64 i128 isize
    f32 f64
);
