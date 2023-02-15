#![doc = include_str!("../spec/var_int.md")]

use crate::*;
use std::{
    convert::{Infallible, TryFrom},
    fmt,
};

macro_rules! def {
    [$name:ident($ty:ty), BITS: $BITS:literal, TryFromErr: $err: ty, $encode:item, $decode:item] => {
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
