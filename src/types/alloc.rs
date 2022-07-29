use crate::*;
use len::Len;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        impl Encoder for $ty {
            #[inline]
            fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.as_ref();
                Len::SIZE + bytes.len()
            }
            #[inline]
            fn encoder(&self, c: &mut impl Array<u8>) {
                let len = self.len().try_into().expect("Invalid length type");
                Len(len).encoder(c);
                c.extend_from_slice(self);
            }
    })*};
}
impls!(Encoder for &[u8], &str, String);

impl<'de> Decoder<'de> for &'de [u8] {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
        let len = Len::decoder(c)?.0;
        c.read_slice(len as usize).ok_or("Insufficient bytes")
    }
}
impl<'de> Decoder<'de> for &'de str {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
        let bytes = Decoder::decoder(c)?;
        core::str::from_utf8(bytes).map_err(|_| "Invalid UTF-8 slice")
    }
}
impl Decoder<'_> for String {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, &'static str> {
        let bytes: &[u8] = Decoder::decoder(c)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "Invalid UTF-8 string")
    }
}

impl<T: Encoder> Encoder for Vec<T> {
    #[inline]
    fn size_hint(&self) -> usize {
        Len::SIZE + self.iter().map(T::size_hint).sum::<usize>()
    }

    #[inline]
    fn encoder(&self, c: &mut impl Array<u8>) {
        let len = self.len().try_into().expect("Invalid length type");
        Len(len).encoder(c);
        
        for item in self {
            item.encoder(c);
        }
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for Vec<T> {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
        let len = Len::decoder(c)?.0;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decoder(c)?);
        }
        Ok(vec)
    }
}

// ---------------------------------------------------------------------

impl<T: Encoder> Encoder for Box<T> {
    const SIZE: usize = size_of::<T>();
    fn encoder(&self, c: &mut impl Array<u8>) {
        T::encoder(self, c);
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for Box<T> {
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
        Ok(Box::new(T::decoder(c)?))
    }
}
