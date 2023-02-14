use crate::*;

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
                let slice = reader.get_unchecked(0);
                *reader = reader.get_unchecked(1..);
                Ok(*slice)
            }
        } else {
            Err("Insufficient bytes".into())
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
macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl Encode for $rty {
            #[inline] fn encode<const CONFIG: u8>(&self, writer: &mut impl Write) -> io::Result<()> {
                match CONFIG & config::num::GET {
                    config::num::LE => writer.write_all(&self.to_le_bytes()),
                    config::num::BE => writer.write_all(&self.to_be_bytes()),
                    config::num::NE => writer.write_all(&self.to_ne_bytes()),
                    _ => unreachable!()
                }
            }
        }
        impl Decode<'_> for $rty {
            #[inline] fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
                let bytes = <&[u8; std::mem::size_of::<Self>()]>::decode::<CONFIG>(c)?;
                Ok(match CONFIG & config::num::GET {
                    config::num::LE => Self::from_le_bytes(*bytes),
                    config::num::BE => Self::from_be_bytes(*bytes),
                    config::num::NE => Self::from_ne_bytes(*bytes),
                    _ => unreachable!()
                })
            }
        }
    )*);
}

impl_data_type_for!(
    u16 u32 u64 u128
    i16 i32 i64 i128
    usize isize
    f32 f64
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler_type() {
        for word in [0x_A5C11, 0x_C0DE, 0x_DEC0DE, 0x_ADDED, 0x_AB0DE, 0x_CAFE] {
            let bytes = word.to_bytes::<{config::DEFAULT}>();
            assert_eq!(word, u32::from_bytes::<{config::DEFAULT}>(&bytes).unwrap());
        }
        for word in [
            0x_DEAD_BEEF,
            0x_Faded_Face,
            0x_BAD_F00D,
            0x_C01D_C0FFEE,
            0x_C0CA_C01A,
        ] {
            let bytes = word.to_bytes::<{config::DEFAULT}>();
            assert_eq!(word, u64::from_bytes::<{config::DEFAULT}>(&bytes).unwrap());
        }
    }
}
