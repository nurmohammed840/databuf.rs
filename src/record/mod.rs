// #[cfg(feature = "nightly")]
// mod specialize;

mod collection;
mod string;

use crate::*;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// [Record](https://docs.rs/bin-layout/latest/bin_layout/struct.Record.html) can be used to
/// encode collections where the size of the length is known.
///
/// For example, `Record<u8, String>` here the maximum allowed payload length is 255 (`u8::MAX`)
///
/// ### Example
///
/// ```rust
/// use bin_layout::*;
///
/// let record: Record<u8, String> = "very long string!".repeat(15).into();
/// let bytes = record.encode();
/// assert_eq!(record.len(), 255);
/// assert_eq!(bytes.len(), 256);
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

/// Supported length type for [Record](https://docs.rs/bin-layout/latest/bin_layout/struct.Record.html)
pub trait LenType:
    fmt::Display + TryFrom<usize> + TryInto<usize> + Encoder + for<'de> Decoder<'de>
{
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
        let len = $data.len();
        Len::try_from(len).map_err(|_| invalid_input(format!("Max payload length is {} ({}), But got {len}", Len::max(), Len::ty_str())))?.encoder($c)?;
    };
}
macro_rules! decode_len {
    [$c: expr] => ({
        let len: usize = Len::decoder($c)?.try_into().map_err(|_| DynErr::from("Invalid length"))?;
        len
    });
}
pub(crate) use decode_len;
pub(crate) use encode_len;
