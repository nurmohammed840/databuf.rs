#![allow(warnings)]
#![cfg_attr(feature = "nightly", feature(array_try_map))]
#![doc = include_str!("../README.md")]

// pub mod utils;

// use core::convert::TryInto;

pub use data_view::{DataView, View};
pub use derive::*;

/// Shortcut for `Result<T, bin_layout::ErrorKind>`
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
pub trait DataType<'a>: Sized {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>);
    fn deserialize(view: &'a mut DataView<impl AsRef<[u8]>>) -> Result<Self>;
}

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl DataType<'_> for $rty {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { view.write(self).unwrap(); }
            #[inline]
            fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self>{ Ok(map!(@opt view.read(); NotEnoughData)) }
        }
    )*);
    [@typle: $(($($name: ident : $idx: tt),*)),*] => ($(
        impl<$($name: DataType,)*> DataType for ($($name,)*) {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { $(self.$idx.serialize(view);)* }
            #[inline]
            fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> { Ok(($($name::deserialize(view)?),*)) }
        }
    )*)
}

// impl_data_type_for!(
//     u8 u16 u32 u64 u128
//     i8 i16 i32 i64 i128
//     usize isize
//     f32 f64
// );

impl_data_type_for!(
    @typle:
        // ()
        // (A:0, B:1)
        // (A:0, B:1, C:2),
        // (A:0, B:1, C:2, D:3),
        // (A:0, B:1, C:2, D:3, E:4)
);

// impl DataType for bool {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         view.write(self as u8).unwrap();
//     }
//     fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         let num: u8 = map!(@opt view.read(); NotEnoughData);
//         Ok(num != 0)
//     }
// }

// impl DataType for String {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         view.write::<u32>(self.len().try_into().unwrap()).unwrap(); // length
//         view.write_slice(self).unwrap();
//     }
//     fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         let len = map!(@opt view.read::<u32>(); NotEnoughData) as usize;
//         let bytes = map!(@opt view.read_slice(len); NotEnoughData).into();
//         Ok(map!(@err String::from_utf8(bytes); InvalidValue))
//     }
// }

// impl<D: DataType, const N: usize> DataType for [D; N] {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         for item in self {
//             item.serialize(view);
//         }
//     }
//     fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         #[cfg(feature = "nightly")]
//         return [(); N].try_map(|_| D::deserialize(view));
//         #[cfg(not(feature = "nightly"))]
//         return (0..N)
//             .map(|_| D::deserialize(view))
//             .collect::<Result<Vec<_>>>()
//             .map(|v| unsafe { v.try_into().unwrap_unchecked() });
//     }
// }

// impl<D: DataType> DataType for Vec<D> {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         view.write::<u32>(self.len().try_into().unwrap()).unwrap(); // length
//         for item in self {
//             item.serialize(view);
//         }
//     }
//     fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         let len = map!(@opt view.read::<u32>(); NotEnoughData);
//         (0..len).map(|_| D::deserialize(view)).collect()
//     }
// }

// struct Bar {
//     data: [u8; 8],
// }
// trait Foo<'a> {
//     fn foo(bar: &'a mut Bar) -> Self;
// }
// impl<'a, 'b, 'c, A, B> Foo<'c> for (A, B)
// where
//     A: Foo<'a>,
//     B: Foo<'b>,
//     'c: 'a + 'b,
// {
//     fn foo(bar: &'c mut Bar) -> Self {
//         (A::foo(bar), B::foo(bar))
//     }
// }