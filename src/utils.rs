use crate::*;
use std::iter::FromIterator;

#[inline]
pub fn invalid_input(error: impl Into<Error>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, error)
}

#[inline]
pub fn get_slice<'de>(remaining: &mut &'de [u8], len: usize) -> Result<&'de [u8]> {
    if len <= remaining.len() {
        unsafe {
            let slice = remaining.get_unchecked(..len);
            *remaining = remaining.get_unchecked(len..);
            Ok(slice)
        }
    } else {
        Err(Box::new(error::InsufficientBytes))
    }
}

#[inline]
pub fn try_collect<'de, T, I, const CONFIG: u16>(cursor: &mut &'de [u8], len: usize) -> Result<I>
where
    T: Decode<'de>,
    I: FromIterator<T>,
{
    let mut error = None;
    let out = I::from_iter(Iter::<T, CONFIG> {
        len,
        err: &mut error,
        reader: cursor,
        _marker: std::marker::PhantomData,
    });
    match error {
        Some(err) => Err(err),
        None => Ok(out),
    }
}

pub struct Iter<'err, 'cursor, 'de, T, const CONFIG: u16> {
    len: usize,
    err: &'err mut Option<Error>,
    reader: &'cursor mut &'de [u8],
    _marker: std::marker::PhantomData<T>,
}

impl<'err, 'cursor, 'de, T, const CONFIG: u16> Iterator for Iter<'err, 'cursor, 'de, T, CONFIG>
where
    T: Decode<'de>,
{
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        match T::decode::<CONFIG>(self.reader) {
            Ok(val) => {
                self.len -= 1;
                Some(val)
            }
            Err(err) => {
                self.len = 0;
                *self.err = Some(err);
                None
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
