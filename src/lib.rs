#![allow(unused_variables)]
#![doc = include_str!("../README.md")]
use std::convert::TryInto;

pub use data_view::{DataView, View};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid,
    Unsupported,
    NotEnoughData,
}

pub trait DataType {
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>);
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! def {
    [$name:ident, { $($field_name:ident : $field_type: ty),* $(,)? }] => {
        #[derive(Debug)]
        struct $name { $($field_name: $field_type,)* }
        impl $crate::DataType for $name {
            fn serialize<T: AsMut<[u8]>>(&self, view: &mut $crate::DataView<T>) {
                $(self.$field_name.serialize(view);)*
            }
            fn deserialize<T: AsRef<[u8]>>(view: &mut $crate::DataView<T>) -> Self {
                Self { $($field_name: $crate::DataType::deserialize(view),)* }
            }
        }
    };
}
macro_rules! not_enough_data {
    [$item: expr] => { match $item { Some(v) => v, None => return Err(Error::NotEnoughData) } };
}
macro_rules! impl_data_type_for {
    [$($rty:ty : $nbyte:literal)*] => ($(
        impl DataType for $rty {
            #[inline]
            fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) { view.write(*self) }
            #[inline]
            fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error>{
                Ok(not_enough_data!(view.read()))
            }
        }
    )*);
    [@typle: $(($($name: ident : $idx: tt),*)),*] => ($(
        impl<$($name: DataType,)*> DataType for ($($name,)*) {
            fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) { $(self.$idx.serialize(view);)* }
            fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error> {
                Ok(($($name::deserialize(view)?),*))
            }
        }
    )*)
}
impl_data_type_for!(u8:1 u16:2 u32:4 u64:8 u128:16 i8:1 i16:2 i32:4 i64:8 i128:16 f32:4 f64:8);
impl_data_type_for!(
    @typle:
        (),
        (A:0, B:1),
        (A:0, B:1, C:2),
        (A:0, B:1, C:2, D:3),
        (A:0, B:1, C:2, D:3, E:4)
);

impl<const N: usize> DataType for [u8; N] {
    #[inline]
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write_slice(self);
    }
    #[inline]
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error> {
        Ok(not_enough_data!(view.read_buf()))
    }
}

impl DataType for bool {
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write(*self as u8)
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error> {
        match view.read::<u8>() {
            Some(n) => Ok(n != 0),
            None => Err(Error::NotEnoughData),
        }
    }
}

impl DataType for String {
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write::<u32>(self.len().try_into().unwrap()); // length
        view.write_slice(self);
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error> {
        let len = not_enough_data!(view.read::<u32>()) as usize;
        let bytes = not_enough_data!(view.read_slice(len)).into();
        match String::from_utf8(bytes) {
            Ok(data) => Ok(data),
            Err(err) => Err(Error::Invalid),
        }
    }
}
impl DataType for Vec<u8> {
    fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
        view.write::<u32>(self.len().try_into().unwrap()); // length
        view.write_slice(self);
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self, Error> {
        let len = not_enough_data!(view.read::<u32>()) as usize;
        Ok(not_enough_data!(view.read_slice(len)).into())
    }
}
