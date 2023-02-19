use crate::*;
use std::ops::{Range, RangeInclusive};

impl<T: Encode> Encode for Range<T> {
    #[inline] fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        self.start.encode::<CONFIG>(c)?;
        self.end.encode::<CONFIG>(c)
    }
}

impl<'de, T: Decode<'de>> Decode<'de> for Range<T> {
    #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        let start = T::decode::<CONFIG>(c)?;
        let end = T::decode::<CONFIG>(c)?;
        Ok(start..end)
    }
}

impl<T: Encode> Encode for RangeInclusive<T> {
    #[inline] fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        self.start().encode::<CONFIG>(c)?;
        self.end().encode::<CONFIG>(c)
    }
}

impl<'de, T: Decode<'de>> Decode<'de> for RangeInclusive<T> {
    #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        let start = T::decode::<CONFIG>(c)?;
        let end = T::decode::<CONFIG>(c)?;
        Ok(start..=end)
    }
}
