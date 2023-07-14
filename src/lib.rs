#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
// #![cfg_attr(feature = "nightly", feature(min_specialization))]

pub use databuf_derive::*;
/// contains configuration options.
pub mod config;
/// This module defines the error types.
pub mod error;
/// This module provides types for encoding and decoding variable-length integers
pub mod var_int;

mod record;
mod types;
mod utils;

use std::{io, io::Write};

/// It is an alias for a boxed [std::error::Error].
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// It is an alias for a `Result<T, Error>` type.
///
/// `Error` is an alias for boxed [std::error::Error] that may occur during [Decode::decode] operation.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// This trait used to serialize the data structure into binary format.
pub trait Encode {
    /// Serialize the data into binary format.
    fn encode<const CONFIG: u16>(&self, _: &mut (impl Write + ?Sized)) -> io::Result<()>;

    /// This is a convenient method used to encode a value into binary data and return it as a [Vec<u8>].
    ///
    /// ### Example
    ///
    /// ```
    /// use databuf::{Encode, config::num::LE};
    ///
    /// #[derive(Encode)]
    /// struct FooBar {
    ///     foo: u8,
    ///     bar: [u8; 2],
    /// }
    /// let bytes = FooBar { foo: 1, bar: [2, 3] }.to_bytes::<LE>();
    /// assert_eq!(bytes, vec![1, 2, 3]);
    /// ```
    #[inline]
    fn to_bytes<const CONFIG: u16>(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encode::<CONFIG>(&mut vec).unwrap();
        vec
    }
}

/// This trait used to deserialize the data structure from binary format.
pub trait Decode<'de>: Sized {
    /// Deserialize the data from binary format.
    fn decode<const CONFIG: u16>(_: &mut &'de [u8]) -> Result<Self>;

    /// This is a convenient method used to decode a value from slice.
    ///
    /// ### Example
    ///
    /// ```
    /// use databuf::{Decode, config::num::LE};
    ///
    /// #[derive(Decode, PartialEq, Debug)]
    /// struct FooBar {
    ///     foo: u8,
    ///     bar: [u8; 2],
    /// }
    ///
    /// let foobar = FooBar::from_bytes::<LE>(&[1, 2, 3]).unwrap();
    /// assert_eq!(foobar, FooBar { foo: 1, bar: [2, 3] });
    /// ```
    #[inline]
    fn from_bytes<const CONFIG: u16>(bytes: &'de [u8]) -> Result<Self> {
        let mut reader = bytes;
        Decode::decode::<CONFIG>(&mut reader)
    }
}

/// Instead of borrowing the data returns owned value.
///
/// This trait is automatically implemented for any type that implements the [Decode] trait.
pub trait DecodeOwned: for<'de> Decode<'de> {}
impl<T> DecodeOwned for T where T: for<'de> Decode<'de> {}
