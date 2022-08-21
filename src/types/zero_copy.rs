use super::*;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        #[cfg(feature = "sizehint")]
        impl SizeHint for $ty {
            #[inline] fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.as_ref();
                len::Len::SIZE + bytes.len()
            }
        }

        impl Encoder for $ty {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> {
                encode_len!(c, self.len());
                c.write_all(self.as_ref())
            }
        }
    )*};
}
impls!(Encoder for &[u8], &str, String);

// impl<'de> Decoder<'de> for &'de [u8] {
//     #[inline]
//     fn decoder(c: &mut &'de [u8]) -> Result<Self> {
//         let len: usize = Len::decoder(c)?.into_inner().try_into().unwrap();
//         get_slice(c, len)
//     }
// }

// impl<'de> Decoder<'de> for &'de str {
//     #[inline]
//     fn decoder(c: &mut &'de [u8]) -> Result<Self> {
//         std::str::from_utf8(Decoder::decoder(c)?).map_err(invalid_data)
//     }
// }

// impl Decoder<'_> for String {
//     #[inline]
//     fn decoder(c: &mut &[u8]) -> Result<Self> {
//         String::from_utf8(<&[u8]>::decoder(c)?.to_vec()).map_err(invalid_data)
//     }
// }