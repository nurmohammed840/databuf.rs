use super::*;
use crate::view::{DataView, Endian};
use core::{
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use ErrorKind::*;

/// By default, `String`, &str, `Vec<T>` ect..  are encoded with their length value first,
/// Default size of length value is 4 bytes (`u32`)
///
/// This utility struct allow you to use different length, For example: `u8`, `u16`, `usize` etc...
///
/// ### Example
///
/// ```rust
/// use bin_layout::{DataType, DataView, Record};
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
pub struct Record<L, T> {
    pub data: T,
    _marker: PhantomData<L>,
}

impl<Len, Type> Record<Len, Type> {
    #[inline]
    const fn new(data: Type) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------------

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de, L> DataType<'de> for Record<L, $($tys)*>
        where
            L: Endian + TryFrom<usize>,
            L::Error: Debug,
            usize: TryFrom<L>,
        {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
                let len: L = self.data.len().try_into().unwrap();
                view.write(len).unwrap(); // length

                view.write_slice(self.data).unwrap();
            }

            #[inline]
            $deserialize
        }
    };
}

impls!(&, 'de, [u8] => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    let len = read_len(view)?;
    view.read_slice(len).map(|bytes| bytes.into()).ok_or(InsufficientBytes)
});

impls!(String => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    let bytes: Record<L, &[u8]> = Record::deserialize(view)?;

    String::from_utf8(bytes.to_vec())
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impls!(&, 'de, str => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    let bytes: Record<L, &[u8]> = Record::deserialize(view)?;

    core::str::from_utf8(bytes.data)
        .map(|string| string.into())
        .map_err(|_| InvalidUtf8)
});

impl<'de, L, T> DataType<'de> for Record<L, Vec<T>>
where
    T: DataType<'de>,
    L: Endian + TryFrom<usize>,
    L::Error: Debug,
    usize: TryFrom<L>,
{
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: L = self.data.len().try_into().unwrap();
        view.write(len).unwrap(); // length

        for record in self.data {
            record.serialize(view);
        }
    }

    #[inline]
    fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
        let len = read_len(view)?;
        (0..len)
            .map(|_| T::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|vec| vec.into())
    }
}

// ---------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------

#[inline]
fn read_len<L: Endian>(view: &mut DataView<&[u8]>) -> Result<usize>
where
    usize: TryFrom<L>,
{
    view.read::<L>()
        .ok_or(InsufficientBytes)
        .and_then(|num| num.try_into().map_err(|_| InvalidLength))
}
