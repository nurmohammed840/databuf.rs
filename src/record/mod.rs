// #[cfg(feature = "nightly")]
// mod specialize;

mod collection;
mod string;

use crate::*;
use std::{
    convert::TryInto,
    fmt,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

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
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

// -----------------------------------------------------------------------

pub trait LenType: TryFrom<usize> + TryInto<usize> + Encoder + for<'de> Decoder<'de> {
    fn max() -> Self;
    fn bits() -> u32;
    fn ty_str() -> &'static str;
}
macro_rules! impl_len_ty {
    [$($ty:ty),*] => {$(
        impl LenType for $ty {
            #[inline] fn max() -> Self { <$ty>::MAX }
            #[inline] fn bits() -> u32 { <$ty>::BITS }
            #[inline] fn ty_str() -> &'static str { stringify!($ty) }
        }
    )*};
}
impl_len_ty!(u8, u16, u32, u64, usize, L2, L3);

macro_rules! encode_len {
    [$data:expr, $c: expr] => {
        let len: Len = $data.len().try_into().map_err(|_| invalid_input("Invalid length"))?;
        len.encoder($c)?;
    };
}
macro_rules! decode_len {
    [$c: expr] => ({
        let len: usize = Len::decoder($c)?.try_into().map_err(|_| invalid_data("Invalid length"))?;
        len
    });
}

pub(crate) use decode_len;
pub(crate) use encode_len;