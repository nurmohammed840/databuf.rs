//! #### Variable-Length Integer Encoding
//!
//! Support types are [BEU15], [BEU22], [BEU29], [BEU30]
//! By default, length of collections is represented with [BEU30].
//!
//! Encoding algorithm is very straightforward,
//! The most significant bits of the first byte determine the byte length to encode the number in big endian.
//!
//! For example, Binary representation of `0x_C0DE` is `0x11000000_11011110`
//!
//! `BEU22(0x_C0DE)` is encoded in 3 bytes:
//!
//! ```yml
//! 1st byte:  11           # MSB is 11, so read next 2 bytes
//! 2nd byte:  11000000
//! 3rd byte:  11011110
//! ```
//!
//! `BEU22(107)` is encoded in just 1 byte:
//!
//! ```yml
//! 1st byte: 0b01101011    # MSB is 0, So we don't have to read extra bytes.
//! ```

use crate::*;
use std::{
    convert::{Infallible, TryFrom},
    fmt,
};

macro_rules! def {
    [$(#[$doc:meta])* $name:ident($ty:ty), BITS: $BITS:literal, UsizeTryFromErr: $err: ty, $encode:item, $decode:item] => {
        $(#[$doc])*
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub $ty);
        impl $name {
            /// The largest value that can be represented by this integer type.
            pub const MAX: $ty = (1 << $BITS) - 1;
            /// The smallest value that can be represented by this integer type.
            pub const MIN: $ty = 0;
            /// The size of this integer type in bits.
            pub const BITS: u32 = $BITS;
        }
        impl Encode for $name { $encode }
        impl Decode<'_> for $name { $decode }
        impl TryFrom<usize> for $name {
            type Error = error::IntegerOverflow;
            #[inline] fn try_from(num: usize) -> std::result::Result<Self, Self::Error> {
                if num > (1 << $BITS) - 1 {
                    Err(error::IntegerOverflow)
                } else {
                    Ok(Self(num as $ty))
                }
            }
        }
        impl TryFrom<$name> for usize {
            type Error = $err;
            #[inline] fn try_from(num: $name) -> std::result::Result<Self, Self::Error> { TryFrom::try_from(num.0) }
        }
        impl From<$ty> for $name { fn from(num: $ty) -> Self { Self(num) } }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
        }
        impl core::ops::Deref for $name {
            type Target = $ty;
            #[inline] fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl core::ops::DerefMut for $name {
            #[inline] fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    };
}

def!(
    /// [BEU15] is variable-length encoder type for non-negative integer values.
    ///
    /// The discriminator of `enum` is represented by [BEU15].
    ///
    /// |  MSB  | Length | Usable Bits | Range    |
    /// | :---: | :----: | :---------: | :------- |
    /// |   0   |   1    |      7      | 0..128   |
    /// |   1   |   2    |     15      | 0..32768 |
    BEU15(u16),
    BITS: 15,
    UsizeTryFromErr: Infallible,
    fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        let num = self.0;
        let b2 = num as u8;
        // (0) 1111111
        if num < (1 << 7) { return c.write_all(&[b2]) }
        debug_assert!(num < (1 << 15));
        let b1 = (num >> 8) as u8;
        // (1) 1111111 11111111
        c.write_all(&[0x80 | b1 , b2])
    },
    fn decode<const CONFIG: u16>(c: &mut &[u8]) -> Result<Self> {
        let b1 = u8::decode::<CONFIG>(c)? as u16;
        // (0) 1111111
        if b1 >> 7 == 0 {
            return Ok(Self(b1))
        }
        let b2 = u8::decode::<CONFIG>(c)? as u16;
        // (1) 1111111 11111111
        Ok(Self(((b1 & 0x7F) << 8) | b2))
    }
);

def!(
    /// [BEU22] is variable-length encoder type for non-negative integer values.
    ///
    /// |  MSB  | Length | Usable Bits | Range      |
    /// | :---: | :----: | :---------: | :--------- |
    /// |  0    |   1    |      7      | 0..128     |
    /// |  10   |   2    |     14      | 0..16384   |
    /// |  11   |   3    |     22      | 0..4194304 |
    BEU22(u32),
    BITS: 22,
    UsizeTryFromErr: std::num::TryFromIntError,
    fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        let num = self.0;
        let b3 = num as u8;
        // (0) 1111111
        if num < (1 << 7) { return c.write_all(&[b3]) }
        let b2 = (num >> 8) as u8;
        // (10) 111111 11111111
        if num < (1 << 14) {
            return c.write_all(&[0x80 | b2, b3])
        }
        // (11) 111111 11111111 11111111
        debug_assert!(num < (1 << 22));
        let b1 = (num >> 16) as u8;
        c.write_all(&[0xC0 | b1, b2, b3])
    },
    fn decode<const CONFIG: u16>(c: &mut &[u8]) -> Result<Self> {
        let b1 = u8::decode::<CONFIG>(c)? as u32;
        // (0) 1111111
        if b1 >> 7 == 0 { return Ok(Self(b1)) }
        // (10) 111111 11111111
        if b1 >> 6 == 0b10 {
            let b2 = u8::decode::<CONFIG>(c)? as u32;
            return Ok(Self((b1 & 0x3F) << 8 | b2))
        }
        // (11) 111111 11111111 11111111
        let [b2, b3] = <&[u8; 2]>::decode::<CONFIG>(c)?;
        let (b2, b3) = (*b2 as u32, *b3 as u32);
        Ok(Self(((b1 & 0x3F) << 16) | (b2 << 8) | b3))
    }
);

def!(
    /// [BEU29] is variable-length encoder type for non-negative integer values.
    ///
    /// |  MSB   | Length | Usable Bits | Range        |
    /// | :---:  | :----: | :---------: | :----------- |
    /// |  0     |   1    |      7      | 0..128       |
    /// |  10    |   2    |     14      | 0..16384     |
    /// |  110   |   3    |     21      | 0..2097152   |
    /// |  111   |   4    |     29      | 0..536870912 |
    BEU29(u32),
    BITS: 29,
    UsizeTryFromErr: std::num::TryFromIntError,
    fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        let num = self.0;
        let b4 = num as u8;
        // (0) 1111111
        if num < (1 << 7) { return c.write_all(&[b4]) }
        let b3 = (num >> 8) as u8; // next 8 bits
        // (10) 111111 11111111
        if num < (1 << 14) {
            return c.write_all(&[0x80 | b3, b4])
        }
        let b2 = (num >> 16) as u8; // next 8 bits
        // (110) 11111 11111111 11111111
        if num < (1 << 21) {
            return c.write_all(&[0xC0 | b2, b3, b4])
        }
        // (111) 11111 11111111 11111111 11111111
        debug_assert!(num < (1 << 29));
        let b1 = (num >> 24) as u8; // next 8 bits
        c.write_all(&[0xE0 | b1, b2, b3, b4])
    },
    fn decode<const CONFIG: u16>(c: &mut &[u8]) -> Result<Self> {
        let b1 = u8::decode::<CONFIG>(c)? as u32;
        // (0) 1111111
        if b1 >> 7 == 0b0 { return Ok(Self(b1)) }
        // (10) 111111 11111111
        if b1 >> 6 == 0b10 {
            let b2 = u8::decode::<CONFIG>(c)? as u32;
            return Ok(Self((b1 & 0x3F) << 8 | b2));
        }
        // (110) 11111  11111111 | 11111111
        if b1 >> 5 == 0b110 {
            let [b2, b3] = <&[u8; 2]>::decode::<CONFIG>(c)?;
            let (b2, b3) = (*b2 as u32, *b3 as u32);
            return Ok(Self((b1 & 0b11111) << 16 | b2 << 8 | b3));
        }
        // (111) 11111 | 11111111 | 11111111 | 11111111
        let [b2, b3, b4] = <&[u8; 3]>::decode::<CONFIG>(c)?;
        let (b2, b3, b4) = (*b2 as u32, *b3 as u32, *b4 as u32);
        Ok(Self((b1 & 0b11111) << 24 | b2 << 16 | b3 << 8 | b4))
    }
);

def!(
    /// [BEU30] is variable-length encoder type for non-negative integer values.
    ///
    /// By default, The length of collections is represented with [BEU30].
    ///
    /// |  MSB  | Length | Usable Bits | Range         |
    /// | :---: | :----: | :---------: | :-----------  |
    /// |  00   |   1    |      6      | 0..64         |
    /// |  01   |   2    |     14      | 0..16384      |
    /// |  10   |   3    |     22      | 0..4194304    |
    /// |  11   |   4    |     30      | 0..1073741824 |
    BEU30(u32),
    BITS: 30,
    UsizeTryFromErr: std::num::TryFromIntError,
    fn encode<const CONFIG: u16>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        let num = self.0;
        let b4 = num as u8;
        // (00) 111111
        if num < (1 << 6) {
            return c.write_all(&[b4])
        }
        let b3 = (num >> 8) as u8;
        // (01) 111111 11111111
        if num < (1 << 14) {
            return c.write_all(&[0x40 | b3, b4])
        }
        let b2 = (num >> 16) as u8;
        // (10) 111111 11111111 11111111
        if num < (1 << 22) {
            return c.write_all(&[0x80 | b2, b3, b4])
        }
        // (11) 111111 11111111 11111111 11111111
        debug_assert!(num < (1 << 30));
        let b1 = (num >> 24) as u8;
        c.write_all(&[0xC0 | b1, b2, b3, b4])
    },

    fn decode<const CONFIG: u16>(c: &mut &[u8]) -> Result<Self> {
        let b1 = u8::decode::<CONFIG>(c)? as u32;
        let len = b1 >> 6;
        // (00) 111111
        if len == 0 { return Ok(Self(b1)) }

        let b1 = b1 & 0x3F;
        // (01) 111111 11111111
        if len == 1 {
            let b2 = u8::decode::<CONFIG>(c)? as u32;
            return Ok(Self(b1 << 8 | b2));
        }
        // (10) 111111 11111111 11111111
        if len == 2 {
            let [b2, b3] = <&[u8; 2]>::decode::<CONFIG>(c)?;
            let (b2, b3) = (*b2 as u32, *b3 as u32);
            return Ok(Self(b1 << 16 | b2 << 8 | b3));
        }
        // (11) 111111 11111111 11111111 11111111
        let [b2, b3, b4] = <&[u8; 3]>::decode::<CONFIG>(c)?;
        let (b2, b3, b4) = (*b2 as u32, *b3 as u32, *b4 as u32);
        Ok(Self(b1 << 24 | b2 << 16 | b3 << 8 | b4))
    }
);
