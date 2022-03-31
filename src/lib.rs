#![allow(warnings)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(array_try_map))]
#![cfg_attr(feature = "auto_traits", feature(auto_traits))]
#![cfg_attr(feature = "auto_traits", feature(negative_impls))]

#[cfg(feature = "auto_traits")]
pub use auto_traits::StaticSized;
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
// pub mod lencoder;
// mod record;
mod types;

use core::convert::TryInto;
use core::mem::{size_of, MaybeUninit};
use core::{fmt, ptr};

// pub use bin_layout_derive::*;
pub use bytes::*;
pub use cursor::*;
// pub use lencoder::Lencoder;
// pub use record::*;

pub trait Error {
    fn insufficient_bytes() -> Self;
    fn invalid_char() -> Self;
}

pub trait Encoder: Sized {
    /// The size of the data type in bytes. (padding not included)
    const SIZE: usize = size_of::<Self>();

    /// Serialize the data to binary format.
    fn encoder(self, _: &mut Cursor<impl Bytes>);

    /// Calculate total estimated size of the data structure in bytes.
    #[inline]
    fn size_hint(&self) -> usize {
        Self::SIZE
    }

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
        let mut cursor = Cursor::new(&mut vec);
        self.encoder(&mut cursor);
        vec
    }
}

pub trait Decoder<'de, E>: Sized {
    /// Deserialize the data from binary format.
    fn decoder(_: &mut Cursor<&'de [u8]>) -> Result<Self, E>;

    /// Shortcut for `DataType::deserialize(&mut Cursor::from(bytes))`
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
    fn decode(data: &'de [u8]) -> Result<Self, E> {
        Self::decoder(&mut Cursor::from(data))
    }
}

// /// A trait for serialize and deserialize data for binary format.
// ///
// /// All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this trait.
// ///
// /// `Vec`, `String`, `&[T]`, `&str` etc.. are encoded with their length value first, Following by each entry.
// pub trait DataType<'de, E>: Sized {
//     /// The size of the data type in bytes. (padding not included)
//     const SIZE: usize = size_of::<Self>();

//     /// If `true`, the data type could be fixed or dynamic sized,
//     ///
//     /// But if `false`, then the data type is always fixed sized.
//     const IS_DYNAMIC: bool = true;

//     /// Serialize the data to binary format.
//     fn serialize(self, _: &mut Cursor<impl Bytes>);

//     /// Deserialize the data from binary format.
//     fn deserialize(_: &mut Cursor<&'de [u8]>) -> Result<Self, E>;

//     /// Calculate total estimated size of the data structure in bytes.
//     #[inline]
//     fn size_hint(&self) -> usize {
//         Self::SIZE
//     }

//     /// ### Example
//     ///
//     /// ```
//     /// use bin_layout::DataType;
//     ///
//     /// #[derive(DataType)]
//     /// struct FooBar {
//     ///     foo: u8,
//     ///     bar: [u8; 2],
//     /// }
//     /// let foobar = FooBar { foo: 1, bar: [2, 3] }.encode();
//     /// assert_eq!(vec![1, 2, 3], foobar);
//     /// ```
//     #[inline]
//     fn encode(self) -> Vec<u8> {
//         let mut vec = Vec::with_capacity(self.size_hint());
//         let mut cursor = Cursor::new(&mut vec);
//         self.serialize(&mut cursor);
//         vec
//     }

//     /// Shortcut for `DataType::deserialize(&mut Cursor::from(bytes))`
//     ///
//     /// ### Example
//     ///
//     /// ```
//     /// use bin_layout::DataType;
//     ///
//     /// #[derive(DataType, PartialEq, Debug)]
//     /// struct FooBar {
//     ///     foo: u8,
//     ///     bar: [u8; 2],
//     /// }
//     ///
//     /// let foobar = FooBar::decode(&[1, 2, 3]).unwrap();
//     /// assert_eq!(foobar, FooBar { foo: 1, bar: [2, 3] });
//     /// ```
//     #[inline]
//     fn decode(data: &'de [u8]) -> Result<Self, E> {
//         Self::deserialize(&mut Cursor::from(data))
//     }
// }
