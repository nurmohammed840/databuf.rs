#![doc = include_str!("../README.md")]
// #![cfg_attr(feature = "nightly", feature(min_specialization))]

pub use bin_layout_derive::*;
pub mod len;
mod record;
mod types;
mod utils;

use len::*;
use std::io::{self, Write};
use utils::*;

pub use record::*;

type DynErr = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, DynErr>;

/// This trait used to serialize the data structure into binary format.
pub trait Encoder {
    /// Serialize the data into binary format.
    fn encoder(&self, _: &mut impl Write) -> io::Result<()>;

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
    /// let bytes = FooBar { foo: 1, bar: [2, 3] }.encode();
    /// assert_eq!(bytes, vec![1, 2, 3]);
    /// ```
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encoder(&mut vec).unwrap();
        vec
    }
}

/// This trait used to deserialize the data structure from binary format.
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
