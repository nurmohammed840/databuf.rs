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
        char::from_u32(num).ok_or_else(|| Error::from(error::InvalidChar))
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

// -----------------------------------------------------------------------------------

macro_rules! zigzag {
    (encode(signed, $this:tt)) => { *$this };
    (encode(unsigned, $this:tt)) => { (($this << 1) ^ ($this >> Self::BITS - 1)) };
    
    (decode(signed, $num:tt)) => { $num };
    (decode(unsigned, $num:tt)) => { (($num >> 1) as Self) ^ -(($num & 1) as Self) };
}

#[rustfmt::skip]
macro_rules! int_to_uint {
    (i16) => { u16 };
    (i32) => { u32 };
    (i64) => { u64 };
    (i128) => { u128 };
    (isize) => { usize };
}

#[rustfmt::skip]
macro_rules! leb128 {
    (@encode: float, $self:tt as $ty:tt, $writer:tt) => { $writer.write_all(&$self.to_le_bytes()) };
    (@encode: signed, $self:tt as $ty:tt, $writer:tt) => { 
        leb128!(encode_signed_or_unsigned($writer, signed, $self as $ty)) 
    };
    (@encode: unsigned, $self:tt as $ty:tt, $writer:tt) => {
        leb128!(encode_signed_or_unsigned($writer, unsigned, $self as int_to_uint!($ty))) 
    };
    (encode_signed_or_unsigned($writer:tt, $catagory:tt, $self:tt as $ty:ty)) => ({
        let mut num = zigzag!(encode($catagory, $self)) as $ty;
        while num > 0b0111_1111 {
            $writer.write_all(&[num as u8 | 0b1000_0000])?;
            num >>= 7;
        }
        $writer.write_all(&[num as u8])
    });

    (@decode: float, $ty:tt, $c:tt) => { Self::from_le_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>($c)?) };
    (@decode: signed, $ty:tt, $c:tt) => { leb128!(decode_signed_or_unsigned(signed, $ty, $c)) };
    (@decode: unsigned, $ty:tt, $c:tt) => { leb128!(decode_signed_or_unsigned(unsigned, int_to_uint!($ty), $c)) };
    (decode_signed_or_unsigned($catagory:tt, $ty:ty, $c:tt)) => ({
        let mut shift: u8 = 0;
        let mut num = 0;
        loop {
            let byte = u8::decode::<CONFIG>($c)?;
            if match Self::BITS {
                16  => shift == 14  && byte > 0b11,
                32  => shift == 28  && byte > 0b1111,
                64  => shift == 63  && byte > 0b1,
                128 => shift == 126 && byte > 0b11,
                _ => unreachable!()
            } {
                return Err(Box::new(error::IntegerOverflow));
            }
            num |= ((byte & 0b0111_1111) as $ty) << shift;
            if (byte & 0b1000_0000) == 0 {
                break zigzag!(decode($catagory, num));
            }
            shift += 7;
        }
    });
}

macro_rules! impl_data_type_for {
    [$catagory:tt => $($num:tt)*] => ($(
        impl Encode for $num {
            fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
                match CONFIG & config::num::GET {
                    config::num::LE => writer.write_all(&self.to_le_bytes()),
                    config::num::BE => writer.write_all(&self.to_be_bytes()),
                    config::num::NE => writer.write_all(&self.to_ne_bytes()),
                    config::num::LEB128 => leb128!(@encode: $catagory, self as $num, writer),
                    _ => unreachable!()
                }
            }
        }
        impl Decode<'_> for $num {
            fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
                Ok(match CONFIG & config::num::GET {
                    config::num::LE => Self::from_le_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::BE => Self::from_be_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::NE => Self::from_ne_bytes(*<&[u8; size_of::<Self>()]>::decode::<CONFIG>(c)?),
                    config::num::LEB128 => leb128!(@decode: $catagory, $num, c),
                    _ => unreachable!()
                })
            }
        }
    )*);
}
impl_data_type_for!(signed => u16 u32 u64 u128 usize);
impl_data_type_for!(unsigned => i16 i32 i64 i128 isize);
impl_data_type_for!(float => f32 f64);

#[cfg(test)]
mod tests {
    #![allow(warnings)]
    use crate::config::num::LEB128;

    use super::*;
    const A: u32 = 15;
 
    #[test]
    fn test_name() {
        let data = u32::MAX.to_bytes::<LEB128>();
        println!("{:?}", data);

        let data = vec![255, 255, 0b11];
        println!("{:?}", u16::from_bytes::<LEB128>(&data));
    }
}
