//! ### Variable-Length Integer Encoding
//!
//! This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `L2` and `L3`.
//!
//! By default, `L2` (u15) is used to encode length (integer) for record. But you override it by setting `L3` (u22) in features flag.
//!  
//! Encoding algorithm is very straightforward, reserving one or two most significant bits of the first byte to encode rest of the length.
//!
//! #### L2
//!
//! |  MSB  | Length | Usable Bits | Range    |
//! | :---: | :----: | :---------: | :------- |
//! |   0   |   1    |      7      | 0..127   |
//! |   1   |   2    |     15      | 0..32767 |
//!
//!
//! #### L3
//!
//! |  MSB  | Length | Usable Bits | Range      |
//! | :---: | :----: | :---------: | :--------- |
//! |   0   |   1    |      7      | 0..127     |
//! |  10   |   2    |     14      | 0..16383   |
//! |  11   |   3    |     22      | 0..4194303 |
//!
//!  
//! For example, Binary representation of `0x_C0DE` is `0x_11_00000011_011110`
//!  
//! `L3(0x_C0DE)` is encoded in 3 bytes:
//!  
//! ```text
//! 1st byte: 11_011110      # MSB is 11, so read next 2 bytes
//! 2nd byte:        11
//! 3rd byte:        11
//! ```
//!
//! Another example, `L3(107)` is encoded in just 1 byte:
//!
//! ```text
//! 1st byte: 0_1101011      # MSB is 0, So we don't have to read another bytes.
//! ```
use crate::*;

#[cfg(feature = "L2")]
pub use L2 as Len;
#[cfg(not(feature = "L2"))]
pub use L3 as Len;

macro_rules! def {
    [$name:ident($ty:ty), LenSize: $size:literal, MAX: $MAX:literal, $encoder:item, $decoder:item] => {
        #[derive(Default, Debug, Clone, Copy, PartialEq)]
        pub struct $name($ty);
        impl $name {
            pub const SIZE: usize = $size;
            pub const MAX: $ty = $MAX;
            pub const fn new(num: $ty) -> Option<Self> {
                if num > Self::MAX { None }
                else { Some(Self(num)) }
            }
            pub const unsafe fn new_unchecked(num: $ty) -> Self { Self(num) }
            #[inline] pub fn into_inner(self) -> $ty { self.0 }
        }

        impl Encoder for $name {
            #[inline] $encoder
        }
        impl Decoder<'_> for $name { #[inline] $decoder }
        impl From<$ty> for $name { fn from(num: $ty) -> Self { Self(num) } }
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
    L2(u16),
    LenSize: 2,
    MAX: 0x7FFF,
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        let num = self.0;
        let b1 = num as u8;
        // No MSB is set, Bcs `num` is less then `128`
        if num < 128 { c.write_all(&[b1]) }
        else {
            debug_assert!(num <= Self::MAX);
            let b1 = 0x80 | b1; // 7 bits with MSB is set.
            let b2 = (num >> 7) as u8; // next 8 bits
            c.write_all(&[b1, b2])
        }
    },
    fn decoder(c: &mut &[u8]) -> Result<Self> {
        let mut num = u8::decoder(c)? as u16;
        // if MSB is set, read another byte.
        if num >> 7 == 1 {
            let snd = u8::decoder(c)? as u16;
            num = (num & 0x7F) | snd << 7; // num <- add 8 bits
        }
        Ok(Self(num))
    }
);
def!(
    L3(u32),
    LenSize: 3,
    MAX: 0x3FFFFF,
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        let num = self.0;
        let b1 = num as u8;
        if num < 128 { c.write_all(&[b1]) }
        else {
            let b1 = b1 & 0x3F; // read last 6 bits
            let b2 = (num >> 6) as u8; // next 8 bits
            if num < 0x4000 {
                // set first 2 bits  of `b1` to `10`
                c.write_all(&[0x80 | b1, b2])
            }
            else {
                // debug_assert!(num <= Self::MAX);
                let b3 = (num >> 14) as u8; // next 8 bits
                // set first 2 bits  of `b1` to `11`
                c.write_all(&[0xC0 | b1, b2, b3])
            }
        }
    },
    fn decoder(c: &mut &[u8]) -> Result<Self> {
        let num = u8::decoder(c)? as u32;
        // if 1st bit is `0`
        let num = if num >> 7 == 0 { num }
        // and 2nd bit is `0`
        else if num >> 6 == 2 {
            let b2 = u8::decoder(c)? as u32;
            (num & 0x3F) | b2 << 6
        } else  {
            // At this point, only possible first 2 bits are `11`
            let [b2, b3] = <&[u8; 2]>::decoder(c)?;

            (num & 0x3F)  // get last 6 bits
            | (*b2 as u32) << 6     // add 8 bits from 2nd byte
            | (*b3 as u32) << 14    // add 8 bits from 3rd byte
        };
        Ok(Self(num))
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_len {
        [$len: expr, $expect: expr] => {
            let bytes = $len.encode();
            assert_eq!(bytes, $expect);
            assert_eq!($len, Decoder::decode(&bytes).unwrap());
        };
    }

    #[test]
    fn l2() {
        assert_eq!(L2::MAX, (1 << 15) - 1);

        assert_len!(L2(0), [0]);
        assert_len!(L2(127), [127]);

        assert_len!(L2(128), [128, 1]);
        assert_len!(L2(32767), [255, 255]);
    }

    #[test]
    fn l3() {
        assert_eq!(L3::MAX, (1 << 22) - 1);

        assert_len!(L3(0), [0]);
        assert_len!(L3(127), [127]);

        assert_len!(L3(128), [128, 2]);
        assert_len!(L3(16383), [191, 255]);

        assert_len!(L3(16384), [192, 0, 1]);
        assert_len!(L3(4194303), [255, 255, 255]);
    }
}
