use crate::*;
use len::Len;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        impl<Len: LenType> Encoder for Record<Len, &$ty>
        where
            Len::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> { impls!(@Body: self.data, c) }
        }
        impl Encoder for $ty {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> { impls!(@Body: self, c) }
        }
    )*};
    [@Body: $data:expr, $c: expr] => ({
        let len: Len = $data.len().try_into().map_err(invalid_input)?;
        len.encoder($c)?;
        $c.write_all($data.as_ref())
    });
}

impls!(Encoder for [u8], str, String);

type DynErr = Box<dyn std::error::Error + Send + Sync>;

fn read_slice<'de, Len: LenType>(c: &mut &'de [u8]) -> Result<&'de [u8]>
where
    usize: TryFrom<Len>,
    <usize as TryFrom<Len>>::Error: Into<DynErr>,
{
    let len = Len::decoder(c)?.try_into().map_err(invalid_data)?;
    get_slice(c, len)
}

// impl<'de> Decoder<'de> for &'de str {
//     fn decoder(c: &mut &'de [u8]) -> Result<Self> {
//         let data = read_slice::<Len>(c)?;
//         todo!()
//     }
// }
impl<'de, Len: LenType> Decoder<'de> for Record<Len, &'de str>
// where
//     usize: TryFrom<L>,
// <usize as TryFrom<L>>::Error: fmt::Debug,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        todo!()
        // let bytes = <Record<L, &[u8]>>::decoder(c)?;
        // core::str::from_utf8(bytes.data)
        //     .map_err(invalid_data)
        //     .map(Record::new)
    }
}

// impl<'de, Len: LenType> Decoder<'de> for Record<Len, &'de [u8]>
// where
//     usize: TryFrom<Len>,
//     <usize as TryFrom<Len>>::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
// {
//     fn decoder(c: &mut &'de [u8]) -> Result<Self> {
//         let len: usize = Len::decoder(c)?.try_into().map_err(invalid_data)?;
//         get_slice(c, len).map(Record::new)
//     }
// }

// impl<T: Encoder> Encoder for [T] {
//     fn encoder(&self, c: &mut impl Write) -> Result<()> {
//         let len: Len = self.len().try_into().map_err(invalid_input)?;
//         len.encoder(c)?;
//         self.iter().try_for_each(|val| val.encoder(c))
//     }
// }
