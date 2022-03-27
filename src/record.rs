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
/// let mut writer = [0; 16].into();
/// record.serialize(&mut writer);
///
/// // One byte for length `u8` and 10 bytes for string. (Total 11 bytes written)
/// assert_eq!(writer.offset, 11);
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<Len: FixedLenInt, T> {
    pub data: T,
    _marker: PhantomData<Len>,
}

// ---------------------------------------------------------------------------------

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de, Len> DataType<'de> for Record<Len, $($tys)*>
        where
            Len: DataType<'de> + FixedLenInt,
            usize: TryFrom<Len>,
        {
            #[inline]
            fn size_hint(&self) -> usize { self.len() + 8 } // assume: 8 bytes for length

            #[inline]
            fn serialize(self, view: &mut Cursor<impl Bytes>) {
                let len: Len = unsafe { self.data.len().try_into().unwrap_unchecked() };
                len.serialize(view);
                view.write_slice(self.data);
            }
            #[inline]
            $deserialize
        }
    };
}
impls!(&, 'de, [u8] => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let len: usize = Len::deserialize(view)?.try_into().map_err(|_| InvalidLength)?;
    view.read_slice(len).map(|bytes| bytes.into())
});
impls!(String => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<Len, &'de [u8]> = Record::deserialize(view)?;

    String::from_utf8(bytes.to_vec())
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});
impls!(&, 'de, str => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<Len, &'de [u8]> = Record::deserialize(view)?;

    core::str::from_utf8(bytes.data)
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impl<'de, Len, T> DataType<'de> for Record<Len, Vec<T>>
where
    T: DataType<'de>,
    Len: DataType<'de> + FixedLenInt,
    usize: TryFrom<Len>,
{
    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        let len: Len = unsafe { self.data.len().try_into().unwrap_unchecked() };
        len.serialize(view);
        for record in self.data {
            record.serialize(view);
        }
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let len: usize = Len::deserialize(view)?
            .try_into()
            .map_err(|_| InvalidLength)?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::deserialize(view)?);
        }
        Ok(vec.into())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        // assume: 8 bytes for length
        8 + self.iter().map(T::size_hint).sum::<usize>() 
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
