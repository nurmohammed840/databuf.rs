use crate::*;
mod collection;
mod string;

use std::{
    fmt,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait LenType: TryFrom<usize> + Encoder + for<'de> Decoder<'de> {}
impl LenType for u8 {}
impl LenType for u16 {}
impl LenType for u32 {}
impl LenType for u64 {}
impl LenType for usize {}
impl LenType for len::L2 {}
impl LenType for len::L3 {}

/// `Record` can be used to represent fixed-size integer to represent the length of a record.
///
/// It accepts fixed-length unsigned interger type of `N` (`u8`, `u32`, `usize`, etc..) and a generic type of `T` (`Vec<T>`, `String` etc..)
/// ### Example
///
/// ```rust
/// use bin_layout::{Record, Encoder};
///
/// let record: Record<u8, &str> = "HelloWorld".into();
/// assert_eq!(record.len(), 10);
///
/// let bytes = record.encode();
/// assert_eq!(bytes.len(), 11);
/// ```
#[derive(Default, Clone)]
pub struct Record<Len: LenType, T> {
    _marker: PhantomData<Len>,
    pub data: T,
}

impl<Len: LenType, T> Record<Len, T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}

impl<Len: LenType, T> From<T> for Record<Len, T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<Len: LenType, T, V: FromIterator<T>> FromIterator<T> for Record<Len, V> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Record::new(V::from_iter(iter))
    }
}

impl<Len: LenType, T: fmt::Debug> fmt::Debug for Record<Len, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
impl<Len: LenType, T> Deref for Record<Len, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<Len: LenType, T> DerefMut for Record<Len, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

macro_rules! encode_len {
    [$data:expr, $c: expr] => {
        let len: Len = $data.len().try_into().map_err(invalid_input)?;
        len.encoder($c)?;
    };
}
pub(crate) use encode_len;
