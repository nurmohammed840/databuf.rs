use crate::*;
macro_rules! impl_encoder_fn {
    [$($data:tt)*] => (
        #[inline] fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
            encode_len!(self $($data)*, c);
            c.write_all(self $($data)* .as_ref())
        }
    );
}
macro_rules! impl_encoder_for {
    [$($ty:ty; $rec_ty:ty),*] => {$(
        impl Encoder for $ty { impl_encoder_fn!(); }
        impl<Len: LenType> Encoder for $rec_ty { impl_encoder_fn!(.data); }
    )*};
    [$($data:tt)*] => (
        #[inline] fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
            encode_len!(self $($data)*, c);
            c.write_all(self $($data)* .as_ref())
        }
    );
}
impl_encoder_for!(str; Record<Len, &str>, String; Record<Len, String>);

macro_rules! read_slice {
    [$c: expr] => ({
        let len = decode_len!($c);
        get_slice($c, len)
    });
}

impl<'de> Decoder<'de> for String {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        String::from_utf8(data.to_vec()).map_err(DynErr::from)
    }
}

impl<'de, Len: LenType> Decoder<'de> for Record<Len, String> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        String::from_utf8(data.to_vec())
            .map_err(DynErr::from)
            .map(Record::new)
    }
}

impl<'de: 'a, 'a> Decoder<'de> for &'a str {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        std::str::from_utf8(data).map_err(DynErr::from)
    }
}

impl<'de: 'a, 'a, Len: LenType> Decoder<'de> for Record<Len, &'a str> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        core::str::from_utf8(data)
            .map_err(DynErr::from)
            .map(Record::new)
    }
}

//------------------------------------------------------------------------------------------

impl<'de: 'a, 'a> Decoder<'de> for &'a [u8] {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c)
    }
}

impl<'de: 'a, 'a, Len: LenType> Decoder<'de> for Record<Len, &'a [u8]> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c).map(Record::new)
    }
}
