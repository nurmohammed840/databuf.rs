//! ### Variable-Length Integer Encoding
//!
//! This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `LEU15` and `LEU22`, both are encoded in little endian.
//!
//! By default, `LEU22` (u22) is used to encode length (integer) for record. But you override it by setting `LEU15` (u15) in features flag.
//!  
//! Encoding algorithm is very straightforward, reserving one or two most significant bits of the first byte to encode rest of the length.
//!
//! #### LEU15
//!
//! |  MSB  | Length | Usable Bits | Range    |
//! | :---: | :----: | :---------: | :------- |
//! |   0   |   1    |      7      | 0..127   |
//! |   1   |   2    |     15      | 0..32767 |
//!
//! #### LEU22
//!
//! |  MSB  | Length | Usable Bits | Range      |
//! | :---: | :----: | :---------: | :--------- |
//! |  0    |   1    |      7      | 0..127     |
//! |  10   |   2    |     14      | 0..16383   |
//! |  11   |   3    |     22      | 0..4194303 |
//!
//!  
//! For example, Binary representation of `0x_C0DE` is `0x_11_00000011_011110`
//!  
//! `LEU22(0x_C0DE)` is encoded in 3 bytes:
//!  
//! ```yml
//! 1st byte: 11_011110      # MSB is 11, so read next 2 bytes
//! 2nd byte:        11
//! 3rd byte:        11
//! ```
//!
//! Another example, `LEU22(107)` is encoded in just 1 byte:
//!
//! ```yml
//! 1st byte: 0_1101011      # MSB is 0, So we don't have to read extra bytes.
//! ```

use crate::*;
use std::{
    convert::{Infallible, TryFrom},
    fmt,
};

macro_rules! def {
    [$name:ident($ty:ty), BITS: $BITS:literal, TryFromErr: $err: ty, $encode:item, $decode:item] => {
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $ty);
        impl $name {
            pub const MAX: $name = $name((1 << $BITS) - 1);
            pub const BITS: u32 = $BITS;
        }
        impl Encode for $name { $encode }
        impl Decode<'_> for $name { $decode }
        impl TryFrom<usize> for $name {
            type Error = String;
            #[inline] fn try_from(num: usize) -> std::result::Result<Self, Self::Error> {
                if num > (1 << $BITS) - 1 {
                    Err(format!("Max payload length is {}, But got {num}", Self::MAX.0))
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
    LEU15(u16),
    BITS: 15,
    TryFromErr: Infallible,
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        let num = self.0;
        let b1 = num as u8;
        // No MSB is set, Bcs `num` is less then `128`
        if num < 128 { return c.write_all(&[b1]) }

        debug_assert!(num <= 0x7FFF);
        let b1 = 0x80 | b1; // 7 bits with MSB is set.
        let b2 = (num >> 7) as u8; // next 8 bits
        c.write_all(&[b1, b2])
    },
    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        let mut num = u8::decode::<CONFIG>(c)? as u16;
        // if MSB is set, read another byte.
        if num >> 7 == 1 {
            let snd = u8::decode::<CONFIG>(c)? as u16;
            num = (num & 0x7F) | snd << 7; // num <- add 8 bits
        }
        Ok(Self(num))
    }
);

def!(
    LEU22(u32),
    BITS: 22,
    TryFromErr: std::num::TryFromIntError,
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        let num = self.0;
        let b1 = num as u8;
        if num < 128 { return c.write_all(&[b1]) }

        let b1 = b1 & 0x3F; // read last 6 bits
        let b2 = (num >> 6) as u8; // next 8 bits

        if num < (1 << 14) {
            // set first 2 bits  of `b1` to `10`
            return c.write_all(&[0x80 | b1, b2])
        }
        debug_assert!(num < (1 << 22));
        let b3 = (num >> 14) as u8; // next 8 bits
        // set first 2 bits  of `b1` to `11`
        c.write_all(&[0xC0 | b1, b2, b3])
    },
    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        let num = u8::decode::<CONFIG>(c)? as u32;
        // if 1st bit is `0`
        let num = if num >> 7 == 0 { num }
        else if num >> 6 == 2 {
            let b2 = u8::decode::<CONFIG>(c)? as u32;
            (num & 0x3F) | b2 << 6
        } else  {
            // At this point, The first 2 bits (MSB) of `b1` are always `11`
            let [b2, b3] = <&[u8; 2]>::decode::<CONFIG>(c)?;
            (num & 0x3F)  // get last 6 bits
            | (*b2 as u32) << 6     // add 8 bits from 2nd byte
            | (*b3 as u32) << 14    // add 8 bits from 3rd byte
        };
        Ok(Self(num))
    }
);

def!(
    LEU29(u32),
    BITS: 29,
    TryFromErr: std::num::TryFromIntError,
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        let num = self.0;
        let b1 = num as u8;
        // (0) 1111111
        if num < (1 << 7) { return c.write_all(&[b1]) }

        // 11111111 (10) 111111
        if num < (1 << 14) {
            let b1 = b1 & 0b_111111; // read 6 LSB
            let b2 = (num >> 6) as u8; // next 8 bits
            // set 2 MSB of `b1` to `10`
            return c.write_all(&[0x80 | b1, b2])
        }

        // 11111111 11111111 (110) 11111
        let b1 = b1 & 0b_11111; // read 5 LSB
        let b2 = (num >> 5) as u8; // next 8 bits
        let b3 = (num >> 13) as u8; // next 8 bits

        if num < (1 << 21) {
            return c.write_all(&[0xC0 | b1, b2, b3])
        }
        // 11111111 11111111 11111111 (111) 11111
        debug_assert!(num < (1 << 29));

        let b4 = (num >> 21) as u8; // next 8 bits
        c.write_all(&[0xE0 | b1, b2, b3, b4])
    },

    fn decode<const CONFIG: u8>(c: &mut &[u8]) -> Result<Self> {
        let b1 = u8::decode::<CONFIG>(c)? as u32;
        // // if 1st bit is `0`
        if b1 >> 7 == 0b0 { return Ok(Self(b1)) }
        if b1 >> 6 == 0b10 {
            // 11111111 (10) 111111
            let b2 = u8::decode::<CONFIG>(c)? as u32;
            return Ok(Self(b2 << 6 | (b1 & 0b_111111)));
        }

        if b1 >> 5 == 0b110 {
            //    b3    |    b2    |    b1
            // 11111111 | 11111111 | (110) 11111
            let [b2, b3] = <&[u8; 2]>::decode::<CONFIG>(c)?;
            let (b3, b2) = (*b3 as u32, *b2 as u32);
            return Ok(Self((b3 << 13) | (b2 << 5) | (b1 & 0b_11111)));
        }

        // At this point, the first 3 bits (MSB) of `b1` are always `111`
        //
        //     b4   |    b3    |    b2    |     b1
        // 11111111 | 11111111 | 11111111 | (111) 11111

        let [b2, b3, b4] = <&[u8; 3]>::decode::<CONFIG>(c)?;
        let (b4, b3, b2) = (*b4 as u32, *b3 as u32, *b2 as u32);

        Ok(Self((b4 << 21) | (b3 << 13) | (b2 << 5) | (b1 & 0b_11111)))
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_int {
        [$len: expr, $expect: expr] => {
            let bytes = $len.to_bytes::<{config::DEFAULT}>();
            assert_eq!(bytes, $expect);
            assert_eq!($len, Decode::from_bytes::<{config::DEFAULT}>(&bytes).unwrap());
        };
    }

    #[test]
    fn le_u15() {
        assert_int!(LEU15(0), [0]);
        assert_int!(LEU15(127), [127]);

        assert_int!(LEU15(128), [128, 1]);
        assert_int!(LEU15(32767), [255, 255]);
    }

    #[test]
    fn le_u22() {
        assert_int!(LEU22(0), [0]);
        assert_int!(LEU22(127), [127]);

        assert_int!(LEU22(128), [128, 2]);
        assert_int!(LEU22(16383), [191, 255]);

        assert_int!(LEU22(16384), [192, 0, 1]);
        assert_int!(LEU22(4194303), [255, 255, 255]);
    }

    #[test]
    fn le_u29() {
        assert_int!(LEU29(0), [0]);
        assert_int!(LEU29(127), [127]);

        assert_int!(LEU29(128), [128, 2]);
        assert_int!(LEU29(16383), [191, 255]);

        assert_int!(LEU29(16384), [192, 0, 2]);
        assert_int!(LEU29(2097151), [223, 255, 255]);
        
        assert_int!(LEU29(2097152), [224, 0, 0, 1]);
        assert_int!(LEU29(536870911), [255, 255, 255, 255]);
    }
}
