use super::*;
use core::{
    fmt,
    fmt::Debug,
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
pub struct Record<N: FixedLenInt, T> {
    pub data: T,
    _marker: PhantomData<N>,
}

// ---------------------------------------------------------------------------------

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de, N> DataType<'de> for Record<N, $($tys)*>
        where
            N: DataType<'de> + FixedLenInt,
            usize: TryFrom<N>,
        {
            #[inline]
            fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
                let len: N = self.data.len().try_into().map_err(|_| InvalidLength)?;
                len.serialize(view)?;
                view.write_slice(self.data)
            }
            #[inline]
            $deserialize
        }
    };
}
impls!(&, 'de, [u8] => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let len: usize = N::deserialize(view)?.try_into().map_err(|_| InvalidLength)?;
    view.read_slice(len).map(|bytes| bytes.into())
});
impls!(String => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<N, &'de [u8]> = Record::deserialize(view)?;

    String::from_utf8(bytes.to_vec())
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});
impls!(&, 'de, str => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<N, &'de [u8]> = Record::deserialize(view)?;

    core::str::from_utf8(bytes.data)
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impl<'de, N, T> DataType<'de> for Record<N, Vec<T>>
where
    T: DataType<'de>,
    N: DataType<'de> + FixedLenInt,
    usize: TryFrom<N>,
{
    #[inline]
    fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
        let len: N = self.data.len().try_into().map_err(|_| InvalidLength)?;
        len.serialize(view)?;
        for record in self.data {
            record.serialize(view)?;
        }
        Ok(())
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let len: usize = N::deserialize(view)?
            .try_into()
            .map_err(|_| InvalidLength)?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::deserialize(view)?);
        }
        Ok(vec.into())
    }
}

impl<N: FixedLenInt, T> Record<N, T> {
    #[inline]
    pub  fn new(data: T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}
impl<N: FixedLenInt, T: Debug> Debug for Record<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
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
