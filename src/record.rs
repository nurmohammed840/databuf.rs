use super::*;
use core::{
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use data_view::Endian;
use ErrorKind::*;

/// This utility struct use for serialize or deserialize variable length records.
///
/// By default, `String` or `Vec<T>` are encoded with their length value first,
/// Size of size is `u32`
///
/// But this utility struct allow you to encode different length size, for example: `u8`, `u16`, `usize` etc...
///
/// ### Example
///
/// ```rust
/// use bin_layout::{DataType, Record};
///
/// let record: Record<u8, String> = String::from("HelloWorld").into();
/// assert_eq!(record.len(), 10);
///
/// let mut buf = [0; 16].into();
/// DataType::serialize(record, &mut buf);
///
/// // One byte for length, 10 bytes for string
/// assert_eq!(buf.offset, 11);  // 11 bytes written to buffer
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<Len, Type> {
    pub data: Type,
    _marker: PhantomData<Len>,
}

impl<Len, Type> Record<Len, Type> {
    const fn new(data: Type) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}

impl<E> DataType for Record<E, String>
where
    E: Endian + TryFrom<usize>,
    E::Error: Debug,
    usize: TryFrom<E>,
{
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write(E::try_from(self.data.len()).unwrap()).unwrap();
        view.write_slice(self.data).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let num: E = view.read().ok_or(InsufficientBytes)?;
        let len: usize = num.try_into().map_err(|_| InvalidData)?;
        let bytes = view.read_slice(len).ok_or(InsufficientBytes)?.into();

        String::from_utf8(bytes)
            .map_err(|_| InvalidData)
            .map(|string| Self::new(string))
    }
}

impl<E, D> DataType for Record<E, Vec<D>>
where
    D: DataType,
    E: Endian + TryFrom<usize>,
    E::Error: Debug,
    usize: TryFrom<E>,
{
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len = E::try_from(self.data.len()).unwrap();
        view.write(len).unwrap();
        for record in self.data {
            record.serialize(view);
        }
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let num: E = view.read().ok_or(InsufficientBytes)?;
        let len = num.try_into().map_err(|_| InvalidLength)?;

        (0..len)
            .map(|_| D::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|data| Record::new(data))
    }
}

impl<L, T: Debug> Debug for Record<L, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<L, T> From<T> for Record<L, T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<L, T> Deref for Record<L, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<L, T> DerefMut for Record<L, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
