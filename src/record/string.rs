use super::*;

macro_rules! impl_encoder_for {
    [$($ty:ty),*] => {$(
        impl Encode for $ty {
            #[inline] fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
                encode_len!(self, c);
                c.write_all(self.as_ref())
            }
        }
    )*};
}
impl_encoder_for!(str, String);

macro_rules! impl_encoder_for_trait_obj {
    [$($ty:ty);*] => {$(
        impl Encode for $ty {
            #[inline]
            fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
                let string = self.to_string();
                encode_len!(string, c);
                c.write_all(string.as_ref())
            }
        }
    )*};
}
impl_encoder_for_trait_obj!(Box<dyn std::fmt::Display>; Box<dyn std::error::Error>; Box<dyn std::error::Error + Send + Sync>);

macro_rules! read_slice {
    [$c: expr] => ({
        let len = decode_len!($c);
        utils::get_slice($c, len)
    });
}

impl<'de> Decode<'de> for String {
    #[inline]
    fn decode<const CONFIG: u16>(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        String::from_utf8(data.to_vec()).map_err(Error::from)
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a str {
    #[inline]
    fn decode<const CONFIG: u16>(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        std::str::from_utf8(data).map_err(Error::from)
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a [u8] {
    #[inline]
    fn decode<const CONFIG: u16>(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c)
    }
}
