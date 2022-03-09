#![allow(unused_variables)]
#![cfg_attr(feature = "nightly", feature(array_try_map))]
#![doc = include_str!("../README.md")]

pub mod utils;
use core::convert::TryInto;
pub use data_view::{DataView, View};

/// Shortcut for `Result<T, ErrorKind>`
pub type Result<T> = core::result::Result<T, ErrorKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidType,
    InvalidValue,
    InvalidLength,
    Unsupported,
    NotEnoughData,
}

macro_rules! map {
    [@err $item:expr ; $err_ty:tt] => { match $item { Ok(v) => v, _ => return Err(ErrorKind::$err_ty) } };
    [@opt $item:expr ; $err_ty:tt] => { match $item { Some(v) => v, _ => return Err(ErrorKind::$err_ty) } };
}
pub(crate) use map;

/// A trait for serialize and deserialize data for binary format.
///
/// All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this trait.
///
/// And For collection types, `Vec` and `String` are supported. They are encoded with their length `u32` value first, Following by each entry of the collection.
pub trait DataType {
    fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>);
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! def {
    [$name:ident, { $($field_name:ident : $field_type: ty),* $(,)? }] => {
        #[derive(Debug)]
        struct $name { $($field_name: $field_type,)* }
        impl $crate::DataType for $name {
            fn serialize<T: AsMut<[u8]>>(self, view: &mut $crate::DataView<T>) { $(self.$field_name.serialize(view);)* }
            fn deserialize<T: AsRef<[u8]>>(view: &mut $crate::DataView<T>) -> $crate::Result<Self> { Ok(Self { $($field_name: $crate::DataType::deserialize(view)?,)* }) }
        }
    };
}

macro_rules! impl_data_type_for {
    [$($rty:ty : $nbyte:literal)*] => ($(
        impl DataType for $rty {
            #[inline]
            fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) { view.write(self) }
            #[inline]
            fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self>{ Ok(map!(@opt view.read(); NotEnoughData)) }
        }
    )*);
    [@typle: $(($($name: ident : $idx: tt),*)),*] => ($(
        impl<$($name: DataType,)*> DataType for ($($name,)*) {
            #[inline]
            fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) { $(self.$idx.serialize(view);)* }
            #[inline]
            fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> { Ok(($($name::deserialize(view)?),*)) }
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
impl DataType for bool {
    fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) {
        view.write(self as u8)
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> {
        let num: u8 = map!(@opt view.read(); NotEnoughData);
        Ok(num != 0)
    }
}
impl DataType for String {
    fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) {
        view.write::<u32>(self.len().try_into().unwrap()); // length
        view.write_slice(self);
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); NotEnoughData) as usize;
        let bytes = map!(@opt view.read_slice(len); NotEnoughData).into();
        Ok(map!(@err String::from_utf8(bytes); InvalidValue))
    }
}

impl<D: DataType, const N: usize> DataType for [D; N] {
    fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) {
        for item in self {
            item.serialize(view);
        }
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> {
        #[cfg(feature = "nightly")]
        return [(); N].try_map(|_| D::deserialize(view));

        #[cfg(not(feature = "nightly"))]
        return (0..N)
            .map(|_| D::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|v| unsafe { v.try_into().unwrap_unchecked() });
    }
}

impl<D: DataType> DataType for Vec<D> {
    fn serialize<T: AsMut<[u8]>>(self, view: &mut DataView<T>) {
        view.write::<u32>(self.len().try_into().unwrap()); // length
        for item in self {
            item.serialize(view);
        }
    }
    fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); NotEnoughData);
        (0..len).map(|_| D::deserialize(view)).collect()
    }
}
