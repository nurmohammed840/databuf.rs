use crate::*;

impl<T> Encode for Option<T>
where
    T: Encode,
{
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        match self {
            Some(val) => {
                c.write_all(&[1])?;
                val.encode::<CONFIG>(c)
            }
            None => c.write_all(&[0]),
        }
    }
}

impl<'de, T: Decode<'de>> Decode<'de> for Option<T> {
    #[inline]
    fn decode<const CONFIG: u8>(r: &mut &'de [u8]) -> Result<Self> {
        Ok(match bool::decode::<CONFIG>(r)? {
            true => Some(T::decode::<CONFIG>(r)?),
            false => None,
        })
    }
}

impl<T, E> Encode for std::result::Result<T, E>
where
    T: Encode,
    E: Encode,
{
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        match self {
            Ok(val) => {
                c.write_all(&[1])?;
                val.encode::<CONFIG>(c)
            }
            Err(err) => {
                c.write_all(&[0])?;
                err.encode::<CONFIG>(c)
            }
        }
    }
}

impl<'de, T, E> Decode<'de> for std::result::Result<T, E>
where
    T: Decode<'de>,
    E: Decode<'de>,
{
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        Ok(match bool::decode::<CONFIG>(c)? {
            true => Ok(T::decode::<CONFIG>(c)?),
            false => Err(E::decode::<CONFIG>(c)?),
        })
    }
}
