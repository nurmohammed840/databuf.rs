use crate::*;

impl<T: Encoder> Encoder for Option<T> {
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        match self {
            Some(val) => {
                c.write_all(&[1])?;
                val.encoder(c)
            }
            None => c.write_all(&[0]),
        }
    }
}

impl<'de, T: Decoder<'de>> Decoder<'de> for Option<T> {
    fn decoder(r: &mut &'de [u8]) -> Result<Self> {
        Ok(match bool::decoder(r)? {
            true => Some(T::decoder(r)?),
            false => None,
        })
    }
}

impl<T: Encoder, E: Encoder> Encoder for std::result::Result<T, E> {
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        match self {
            Ok(val) => {
                c.write_all(&[1])?;
                val.encoder(c)
            }
            Err(err) => {
                c.write_all(&[0])?;
                err.encoder(c)
            }
        }
    }
}

impl<'de, T, E> Decoder<'de> for std::result::Result<T, E>
where
    T: Decoder<'de>,
    E: Decoder<'de>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        Ok(match bool::decoder(c)? {
            true => Ok(T::decoder(c)?),
            false => Err(E::decoder(c)?),
        })
    }
}
