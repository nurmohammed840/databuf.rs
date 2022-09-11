#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(array_try_map, min_specialization))]
#![feature(min_specialization)]

pub use bin_layout_derive::*;
pub mod len;
mod record;
mod types;
mod utils;

// #[cfg(feature = "nightly")]
mod specialization;

use utils::*;
use len::Len;
use std::io::{Error, ErrorKind, Result, Write};

pub use record::*;

pub trait Encoder {
    /// Serialize the data to binary format.
    fn encoder(&self, _: &mut impl Write) -> Result<()>;

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
    /// assert_eq!(foobar, vec![1, 2, 3]);
    /// ```
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encoder(&mut vec).unwrap();
        vec
    }
}

pub trait Decoder<'de>: Sized {
    /// Deserialize the data from binary format.
    fn decoder(_: &mut &'de [u8]) -> Result<Self>;

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
    /// let foobar = FooBar::decode(&[1, 2, 3]).unwrap();
    /// assert_eq!(foobar, FooBar { foo: 1, bar: [2, 3] });
    /// ```
    #[inline]
    fn decode(data: &'de [u8]) -> Result<Self> {
        let mut reader = data;
        Self::decoder(&mut reader)
    }
}
