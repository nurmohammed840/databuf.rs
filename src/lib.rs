#![allow(warnings)]
#![cfg_attr(feature = "nightly", feature(array_try_map))]
#![doc = include_str!("../README.md")]

mod record;
mod types;

use core::convert::TryInto;

pub use data_view;
pub use data_view::DataView;
pub use derive::*;
pub use record::Record;
/// Shortcut for `Result<T, bin_layout::ErrorKind>`
pub type Result<T> = core::result::Result<T, ErrorKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidType,
    InvalidData,
    InvalidInput,
    InvalidLength,
    Unsupported,
    InsufficientBytes,
    Other,
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
pub trait DataType: Sized {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>);
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self>;
}
