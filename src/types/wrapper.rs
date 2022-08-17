use crate::*;

impl<T> Encoder for std::marker::PhantomData<T> {
    fn size_hint(&self) -> usize {
        0
    }

    fn encoder(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

impl<T> Decoder<'_> for std::marker::PhantomData<T> {
    fn decoder(_: &mut &[u8]) -> Result<Self> {
        Ok(std::marker::PhantomData)
    }
}

//  ------------------------------------------------------------------------

impl<T: Encoder> Encoder for Box<T> {
    fn size_hint(&self) -> usize {
        T::size_hint(self)
    }
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        T::encoder(self, c)
    }
}

impl<'de, T> Decoder<'de> for Box<T>
where
    T: Decoder<'de>,
{
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        T::decoder(c).map(|v| Box::new(v))
    }
}
