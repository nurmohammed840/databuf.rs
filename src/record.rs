use super::*;
use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait FixedLenInt: TryFrom<usize> {}
impl FixedLenInt for u8 {}
impl FixedLenInt for u16 {}
impl FixedLenInt for u32 {}
impl FixedLenInt for u64 {}
impl FixedLenInt for usize {}

/// `Record` can be used to represent fixed-size integer to represent the length of a record.
///
/// It accepts fixed-length unsigned interger type of `N` (`u8`, `u32`, `usize`, etc..) and a generic type of `T` (`Vec<T>`, `String` etc..)
/// ### Example
///
/// ```rust
/// use bin_layout::{Record, DataType};
///
/// let record: Record<u8, &str> = "HelloWorld".into();
/// assert_eq!(record.len(), 10);
///
/// let bytes = record.encode();
/// assert_eq!(bytes.len(), 11);
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<Len: FixedLenInt, T> {
    pub data: T,
    _marker: PhantomData<Len>,
}

// ---------------------------------------------------------------------------------

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        impl<Len: Encoder + FixedLenInt> Encoder for Record<Len, $ty> {
            #[inline]
            fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.as_ref();
                Len::SIZE + bytes.len()
            }
            #[inline]
            fn encoder(self, c: &mut Cursor<impl Bytes>) {
                let len: Len = unsafe { self.data.len().try_into().unwrap_unchecked() };
                len.encoder(c);
                c.write_slice(self.data);
            }
        }
    )*};
}
impls!(Encoder for &[u8], &str, String);

impl<'de, E: Error, Len> Decoder<'de, E> for Record<Len, &'de [u8]>
where
    usize: TryFrom<Len>,
    Len: FixedLenInt + Decoder<'de, E>,
{
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let len: usize = Len::decoder(c)?.try_into().map_err(|_| E::invalid_data())?;
        c.read_slice(len).map(|bytes| bytes.into())
    }
}

impl<'de, E: Error, Len> Decoder<'de, E> for Record<Len, &'de str>
where
    usize: TryFrom<Len>,
    Len: FixedLenInt + Decoder<'de, E>,
{
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let bytes: Record<Len, &'de [u8]> = Record::decoder(c)?;

        core::str::from_utf8(bytes.data)
            .map(|string| string.into())
            .map_err(E::utf8_err)
    }
}

impl<'de, E: Error, Len> Decoder<'de, E> for Record<Len, String>
where
    usize: TryFrom<Len>,
    Len: FixedLenInt + Decoder<'de, E>,
{
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let bytes: Record<Len, &'de [u8]> = Record::decoder(c)?;

        String::from_utf8(bytes.to_vec())
            .map(|string| string.into())
            .map_err(E::from_utf8_err)
    }
}

impl<Len, T> Encoder for Record<Len, Vec<T>>
where
    T: Encoder,
    Len: Encoder + FixedLenInt,
{
    #[inline]
    fn size_hint(&self) -> usize {
        Len::SIZE + self.iter().map(T::size_hint).sum::<usize>()
    }

    #[inline]
    fn encoder(self, c: &mut Cursor<impl Bytes>) {
        let len: Len = unsafe { self.data.len().try_into().unwrap_unchecked() };
        len.encoder(c);

        for record in self.data {
            record.encoder(c);
        }
    }
}

impl<'de, E: Error, Len, T> Decoder<'de, E> for Record<Len, Vec<T>>
where
    T: Decoder<'de, E>,
    usize: TryFrom<Len>,
    Len: Decoder<'de, E> + FixedLenInt,
{
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        let len: usize = Len::decoder(c)?.try_into().map_err(|_| E::invalid_data())?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decoder(c)?);
        }
        Ok(vec.into())
    }
}

impl<N: FixedLenInt, T> Record<N, T> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}
impl<N: FixedLenInt, T: fmt::Debug> fmt::Debug for Record<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
impl<N: FixedLenInt, T> From<T> for Record<N, T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
impl<N: FixedLenInt, T> Deref for Record<N, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<N: FixedLenInt, T> DerefMut for Record<N, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
