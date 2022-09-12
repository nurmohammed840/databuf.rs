use crate::*;
use std::iter::FromIterator;

pub type DynErr = Box<dyn std::error::Error + Send + Sync>;

pub fn invalid_data(error: impl Into<DynErr>) -> Error {
    Error::new(ErrorKind::InvalidData, error)
}
pub fn invalid_input(error: impl Into<DynErr>) -> Error {
    Error::new(ErrorKind::InvalidInput, error)
}

pub fn get_slice<'a>(this: &mut &'a [u8], len: usize) -> Result<&'a [u8]> {
    if len <= this.len() {
        unsafe {
            let slice = this.get_unchecked(..len);
            *this = this.get_unchecked(len..);
            Ok(slice)
        }
    } else {
        Err(Error::new(ErrorKind::UnexpectedEof, "Insufficient bytes"))
    }
}

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

pub struct Iter<'err, 'c, 'de, T> {
    len: usize,
    err: &'err mut Option<Error>,
    cursor: &'c mut &'de [u8],
    _marker: std::marker::PhantomData<T>,
}

impl<'err, 'c, 'de, T: Decoder<'de>> Iterator for Iter<'err, 'c, 'de, T> {
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
