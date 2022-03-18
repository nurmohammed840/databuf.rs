use super::*;
use core::{
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// By default, `String`, &str, `Vec<T>` ect..  are encoded with their length value first,
/// Default size of length value is 4 bytes (`u32`)
///
/// This utility struct allow you to use different length, For example: `u8`, `u16`, `usize` etc...
///
/// ### Example
///
/// ```rust
/// use bin_layout::{DataType, View, Record};
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
pub struct Record<Len, T> {
    pub data: T,
    _marker: PhantomData<Len>,
}

// ---------------------------------------------------------------------------------

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de, L> DataType<'de> for Record<L, $($tys)*>
        where
            L: DataType<'de> + TryFrom<usize>,
            usize: TryFrom<L>,
        {
            #[inline]
            fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
                let len: L = self.data.len().try_into().map_err(|_| InvalidLength)?;
                len.serialize(view)?;
                view.write_slice(self.data)
            }
            #[inline]
            $deserialize
        }
    };
}

impls!(&, 'de, [u8] => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let len: usize = L::deserialize(view)?.try_into().map_err(|_| InvalidLength)?;
    view.read_slice(len).map(|bytes| bytes.into())
});

impls!(String => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<L, &[u8]> = Record::deserialize(view)?;

    String::from_utf8(bytes.to_vec())
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impls!(&, 'de, str => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: Record<L, &[u8]> = Record::deserialize(view)?;

    core::str::from_utf8(bytes.data)
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impl<'de, L, T> DataType<'de> for Record<L, Vec<T>>
where
    T: DataType<'de>,
    L: DataType<'de> + TryFrom<usize>,
    usize: TryFrom<L>,
{
    #[inline]
    fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
        let len: L = self.data.len().try_into().map_err(|_| InvalidLength)?;
        len.serialize(view)?;
        for record in self.data {
            record.serialize(view)?;
        }
        Ok(())
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let len: usize = L::deserialize(view)?
            .try_into()
            .map_err(|_| InvalidLength)?;

        (0..len)
            .map(|_| T::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|vec| vec.into())
    }
}

impl<L, T> Record<L, T> {
    #[inline]
    const fn new(data: T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}
impl<L, T: Debug> Debug for Record<L, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
impl<L, T> From<T> for Record<L, T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
impl<L, T> Deref for Record<L, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<L, T> DerefMut for Record<L, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
