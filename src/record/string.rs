use crate::*;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        impl<Len: LenType> Encoder for Record<Len, &$ty>
        where
            Len::Error: Into<DynErr>,
        {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> { impls!(@Body: self.data, c) }
        }
        impl Encoder for $ty {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> { impls!(@Body: self, c) }
        }
    )*};
    [@Body: $data:expr, $c: expr] => ({
        encode_len!($data, $c);
        $c.write_all($data.as_ref())
    });
}

impls!(Encoder for str, String);

macro_rules! read_slice {
    [$c: expr] => ({
        let len: usize = Len::decoder($c)?.try_into().map_err(invalid_input)?;
        get_slice($c, len)
    });
}

impl<'de> Decoder<'de> for String {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        String::from_utf8(data.to_vec()).map_err(invalid_data)
    }
}

impl<'de, Len: LenType> Decoder<'de> for Record<Len, String>
where
    usize: TryFrom<Len>,
    <usize as TryFrom<Len>>::Error: Into<DynErr>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        // let data = read_slice!(c)?;
        let data = {
            let len: usize = Len::decoder(c)?.try_into().map_err(invalid_input)?;
            get_slice(c, len)
        }?;
        String::from_utf8(data.to_vec())
            .map_err(invalid_data)
            .map(Record::new)
    }
}

impl<'de: 'a, 'a> Decoder<'de> for &'a str {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        std::str::from_utf8(data).map_err(invalid_data)
    }
}

impl<'de: 'a, 'a, Len: LenType> Decoder<'de> for Record<Len, &'a str>
where
    usize: TryFrom<Len>,
    <usize as TryFrom<Len>>::Error: Into<DynErr>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let data = read_slice!(c)?;
        core::str::from_utf8(data)
            .map_err(invalid_data)
            .map(Record::new)
    }
}

impl<'de: 'a, 'a> Decoder<'de> for &'a [u8] {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c)
    }
}

impl<'de: 'a, 'a, Len: LenType> Decoder<'de> for Record<Len, &'a [u8]>
where
    usize: TryFrom<Len>,
    <usize as TryFrom<Len>>::Error: Into<DynErr>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        read_slice!(c).map(Record::new)
    }
}
