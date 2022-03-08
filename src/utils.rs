use super::*;
use core::{
    convert::TryFrom,
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use data_view::Endian;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Records<L, T> {
    data: T,
    _marker: PhantomData<L>,
}

impl<E> DataType for Records<E, String>
where
    E: Endian + TryFrom<usize>,
    E::Error: Debug,
    usize: TryFrom<E>,
    <usize as TryFrom<E>>::Error: Debug,
{
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write(E::try_from(self.data.len()).unwrap());
        view.write_slice(&self.data);
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, ErrorKind> {
        let num: E = map!(@opt view.read(); NotEnoughData);
        let len: usize = map!(@err num.try_into(); InvalidLength);
        let bytes = map!(@opt view.read_slice(len); NotEnoughData).into();
        let string = map!(@err String::from_utf8(bytes); InvalidValue);
        Ok(string.into())
    }
}

impl<D, E> DataType for Records<E, Vec<D>>
where
    D: DataType,
    E: Endian + TryFrom<usize>,
    E::Error: Debug,
    usize: TryFrom<E>,
    <usize as TryFrom<E>>::Error: Debug,
{
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write(E::try_from(self.data.len()).unwrap());
        for record in &self.data {
            record.serialize(view);
        }
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, ErrorKind> {
        let num: E = map!(@opt view.read(); NotEnoughData);
        let len: usize = map!(@err num.try_into(); InvalidLength);
        let records = (0..len)
            .map(|_| D::deserialize(view))
            .collect::<Result<Vec<_>, _>>()?
            .into();

        Ok(records)
    }
}

impl<L, T: Debug> Debug for Records<L, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<L, T> From<T> for Records<L, T> {
    fn from(data: T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}

impl<L, T> Deref for Records<L, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<L, T> DerefMut for Records<L, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
