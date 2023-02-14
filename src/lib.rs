#![doc = include_str!("../README.md")]
#![allow(clippy::unusual_byte_groupings)]
// #![cfg_attr(feature = "nightly", feature(min_specialization))]

pub use databuf_derive::*;
pub mod config;
pub mod var_int;

mod record;
mod types;
mod utils;

use std::{io, io::Write};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// This trait used to serialize the data structure into binary format.
pub trait Encode {
    /// Serialize the data into binary format.
    fn encode<const CONFIG: u8>(&self, _: &mut impl Write) -> io::Result<()>;

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
    fn to_bytes<const CONFIG: u8>(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encode::<CONFIG>(&mut vec).unwrap();
        vec
    }
}

/// This trait used to deserialize the data structure from binary format.
pub trait Decode<'de>: Sized {
    /// Deserialize the data from binary format.
    fn decode<const CONFIG: u8>(_: &mut &'de [u8]) -> Result<Self>;

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
    fn from_bytes<const CONFIG: u8>(data: &'de [u8]) -> Result<Self> {
        let mut reader = data;
        Decode::decode::<CONFIG>(&mut reader)
    }
}
