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
        Err("Insufficient bytes".into())
    }
}

#[inline]
pub fn try_collect<'de, T, I>(cursor: &mut &'de [u8], len: usize) -> Result<I>
where
    T: Decoder<'de>,
    I: FromIterator<T>,
{
    let mut error = None;
    let out = I::from_iter(Iter {
        len,
        err: &mut error,
        cursor,
        _marker: std::marker::PhantomData,
    });
    match error {
        Some(err) => Err(err),
        None => Ok(out),
    }
}

pub struct Iter<'err, 'cursor, 'de, T> {
    len: usize,
    err: &'err mut Option<Error>,
    cursor: &'cursor mut &'de [u8],
    _marker: std::marker::PhantomData<T>,
}

impl<'err, 'cursor, 'de, T: Decoder<'de>> Iterator for Iter<'err, 'cursor, 'de, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        match T::decoder(self.cursor) {
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
