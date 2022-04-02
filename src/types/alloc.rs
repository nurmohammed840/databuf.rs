use crate::*;
use lencoder::Lencoder;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        impl Encoder for $ty {
            #[inline]
            fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.as_ref();
                Lencoder::SIZE + bytes.len()
            }
            #[inline]
            fn encoder(self, view: &mut Cursor<impl Bytes>) {
                Lencoder(self.len().try_into().unwrap()).encoder(view);
                view.write_slice(self);
            }
    })*};
}
impls!(Encoder for &[u8], &str, String);

impl<'de, E: Error> Decoder<'de, E> for &'de [u8] {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let len = Lencoder::decoder(c)?.0;
        c.read_slice(len as usize)
    }
}
impl<'de, E: Error> Decoder<'de, E> for &'de str {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let bytes: &'de [u8] = Decoder::decoder(c)?;
        core::str::from_utf8(bytes).map_err(E::utf8_err)
    }
}
impl<E: Error> Decoder<'_, E> for String {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
        let bytes: &[u8] = Decoder::decoder(c)?;
        String::from_utf8(bytes.to_vec()).map_err(E::from_utf8_err)
    }
}

impl<T: Encoder> Encoder for Vec<T> {
    #[inline]
    fn size_hint(&self) -> usize {
        Lencoder::SIZE + self.iter().map(T::size_hint).sum::<usize>()
    }

    #[inline]
    fn encoder(self, c: &mut Cursor<impl Bytes>) {
        Lencoder(self.len().try_into().unwrap()).encoder(c);
        for item in self {
            item.encoder(c);
        }
    }
}

impl<'de, E: Error, T: Decoder<'de, E>> Decoder<'de, E> for Vec<T> {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let len = Lencoder::decoder(c)?.0;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decoder(c)?);
        }
        Ok(vec)
    }
}
