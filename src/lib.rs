#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(array_try_map))]

pub mod len;
mod types;
// mod record;

use core::mem::size_of;

pub use bin_layout_derive::*;
pub use len::Len;
use std::io::{Error, ErrorKind, Result, Write};
// pub use record::*;

pub trait Encoder: Sized {
    /// The size of the data type in bytes. (padding not included)
    const SIZE: usize = size_of::<Self>();

    /// Serialize the data to binary format.
    fn encoder(&self, _: &mut impl Write) -> Result<()>;

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
    /// assert_eq!(foobar, vec![1, 2, 3]);
    /// ```
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.size_hint());
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

// -------------------------------------------------------------------------------

fn invalid_data<E>(error: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>
{
    Error::new(ErrorKind::InvalidData, error)
}

fn get_slice<'a>(this: &mut &'a [u8], len: usize) -> Result<&'a [u8]> {
    if len <= this.len() {
        unsafe {
            let slice = this.get_unchecked(..len);
            *this = this.get_unchecked(len..);
            Ok(slice)
        }
    } else {
        Err(Error::new(ErrorKind::UnexpectedEof, "Insufficient bytes"))
    }
}
