use crate::*;

impl<T: Encoder> Encoder for &T {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        T::encoder(self, c)
    }
}

impl<T: Encoder> Encoder for &mut T {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        T::encoder(self, c)
    }
}

impl<T> Encoder for std::marker::PhantomData<T> {
    #[inline]
    fn encoder(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

impl<T> Decoder<'_> for std::marker::PhantomData<T> {
    #[inline]
    fn decoder(_: &mut &[u8]) -> Result<Self> {
        Ok(std::marker::PhantomData)
    }
}

impl<T: Encoder> Encoder for Box<T> {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        T::encoder(self, c)
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for Box<T> {
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        T::decoder(c).map(|v| Box::new(v))
    }
}
