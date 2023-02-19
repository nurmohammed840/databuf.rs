use super::*;

macro_rules! impl_encoder_for {
    [$($ty:ty),*] => {$(
        impl Encode for $ty {
            #[inline] fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
                encode_len!(self, c);
                c.write_all(self.as_ref())
            }
        }
    )*};
}
impl_encoder_for!(str, String);

macro_rules! read_slice {
    [$c: expr] => ({
        let len = decode_len!($c);
        utils::get_slice($c, len)
    });
}

impl<'de> Decode<'de> for String {
    #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        String::from_utf8(data.to_vec()).map_err(Error::from)
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a str {
    #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        std::str::from_utf8(data).map_err(Error::from)
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a [u8] {
    #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c)
    }
}
