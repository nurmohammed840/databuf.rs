#![cfg_attr(feature = "nightly", feature(array_try_map))]
#![doc = include_str!("../README.md")]

mod bytes;
mod cursor;
pub mod lencoder;
mod record;
mod types;

use core::convert::TryInto;
use core::mem::{size_of, MaybeUninit};
use core::{fmt, ptr};
use ErrorKind::*;

pub use bytes::*;
pub use cursor::*;
pub use derive::*;
pub use record::*;

/// Shortcut for `Result<T, bin_layout::ErrorKind>`
pub type Result<T> = core::result::Result<T, ErrorKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    InsufficientBytes,
    InvalidLength,
    InvalidInput,
    LenOverflow,
    Unsupported,
    InvalidType,
    InvalidData,
    InvalidChar,
    InvalidUtf8,
    Other,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
impl std::error::Error for ErrorKind {}

/// A trait for serialize and deserialize data for binary format.
///
/// All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this trait.
///
/// `Vec`, `String`, `&[T]`, `&str` etc.. are encoded with their length value first, Following by each entry.
pub trait DataType<'de>: Sized {
    const SIZE: usize = size_of::<Self>();

    #[inline]
    fn size_hint(&self) -> usize {
        Self::SIZE
    }

    /// Serialize the data to binary format.
    fn serialize(self, _: &mut Cursor<impl Bytes>);

    /// Deserialize the data from binary format.
    fn deserialize(_: &mut Cursor<&'de [u8]>) -> Result<Self>;

    /// ### Example
    ///
    /// ```
    /// use bin_layout::DataType;
    ///
    /// #[derive(DataType)]
    /// struct FooBar {
    ///     foo: u8,
    ///     bar: [u8; 2],
    /// }
    ///
    /// let mut bytes = [0; 3];
    /// FooBar { foo: 1, bar: [2, 3] }.encode(&mut bytes);
    /// assert_eq!(bytes, [1, 2, 3]);
    /// ```
    #[inline]
    fn encode(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.size_hint());
        let mut cursor: Cursor<&mut Vec<u8>> = (&mut vec).into();
        self.serialize(&mut cursor);
        vec
    }

    /// Shortcut for `DataType::deserialize(&mut View::new(bytes.as_ref()))`
    ///
    /// ### Example
    ///
    /// ```
    /// use bin_layout::DataType;
    ///
    /// #[derive(DataType, PartialEq, Debug)]
    /// struct FooBar {
    ///     foo: u8,
    ///     bar: [u8; 2],
    /// }
    ///
    /// let foobar = FooBar::decode(&[1, 2, 3]).unwrap();
    /// assert_eq!(foobar, FooBar { foo: 1, bar: [2, 3] });
    /// ```
    #[inline]
    fn decode(data: &'de [u8]) -> Result<Self> {
        Self::deserialize(&mut Cursor::from(data))
    }
}
