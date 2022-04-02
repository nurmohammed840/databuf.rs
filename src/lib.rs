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
pub mod lencoder;
mod record;
mod types;
mod error;

use core::convert::TryInto;
use core::mem::{size_of, MaybeUninit};
use core::{fmt, ptr};

pub use bin_layout_derive::*;
pub use error::*;
pub use bytes::*;
pub use cursor::*;
pub use lencoder::Lencoder;
pub use record::*;

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
    /// use bin_layout::Encoder;
    ///
    /// #[derive(Encoder)]
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

    /// ### Example
    ///
    /// ```
    /// use bin_layout::Decoder;
    ///
    /// #[derive(Decoder, PartialEq, Debug)]
    /// struct FooBar {
    ///     foo: u8,
    ///     bar: [u8; 2],
    /// }
    ///
    /// let foobar: Result<FooBar, ()> = FooBar::decode(&[1, 2, 3]);
    /// assert_eq!(foobar, Ok(FooBar { foo: 1, bar: [2, 3] }));
    /// ```
    #[inline]
    fn decode(data: &'de [u8]) -> Result<Self, E> {
        Self::decoder(&mut Cursor::from(data))
    }
}