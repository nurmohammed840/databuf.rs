use crate::*;
use std::ops::{Range, RangeInclusive};

impl<T: Encoder> Encoder for Range<T> {
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        self.start.encoder(c)?;
        self.end.encoder(c)
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for Range<T> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let start = T::decoder(c)?;
        let end = T::decoder(c)?;
        Ok(start..end)
    }
}

impl<T: Encoder> Encoder for RangeInclusive<T> {
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        self.start().encoder(c)?;
        self.end().encoder(c)
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for RangeInclusive<T> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let start = T::decoder(c)?;
        let end = T::decoder(c)?;
        Ok(start..=end)
    }
}
