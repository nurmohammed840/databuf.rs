#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(array_try_map))]

#![cfg_attr(feature = "auto_traits", feature(auto_traits))]
#![cfg_attr(feature = "auto_traits", feature(negative_impls))]
#[cfg(feature = "auto_traits")]
pub use auto_traits::*;
#[cfg(feature = "auto_traits")]
mod auto_traits {
    pub unsafe auto trait StaticSized {}
    
    impl !StaticSized for &str {}
    impl !StaticSized for String {}
    impl<T> !StaticSized for &[T] {}
    impl<T> !StaticSized for Vec<T> {}
}


mod bytes;
mod cursor;
pub mod lencoder;
mod record;
mod types;

use core::convert::TryInto;
use core::mem::{size_of, MaybeUninit};
use core::{fmt, ptr};
use ErrorKind::*;

pub use bin_layout_derive::*;
pub use bytes::*;
pub use cursor::*;
pub use lencoder::Lencoder;
pub use record::*;

/// Shortcut for `Result<T, bin_layout::ErrorKind>`
pub type Result<T> = core::result::Result<T, ErrorKind>;

/// The kind of error that occurred during encoding or decoding.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// The input data was too short to decode.
    InsufficientBytes,
    /// The input data was too long to decode.
    InvalidLength,
    /// The input data was not in a valid format.
    InvalidInput,
    /// Unsupported type.
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
    /// The size of the data type in bytes. (padding not included)
    const SIZE: usize = size_of::<Self>();

    /// If `true`, the data type could be fixed or dynamic sized,
    ///
    /// But if `false`, then the data type is always fixed sized.
    const IS_DYNAMIC: bool = true;

    /// Calculate total estimated size of the data structure in bytes.
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
    /// let foobar = FooBar { foo: 1, bar: [2, 3] }.encode();
    /// assert_eq!(vec![1, 2, 3], foobar);
    /// ```
    #[inline]
    fn encode(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.size_hint());
        self.serialize(&mut (&mut vec).into());
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
