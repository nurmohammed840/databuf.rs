#![allow(warnings)]
use crate::*;

macro_rules! def {
    [$name:ident($ty:ty), MAX: $MAX:literal, $serialize:item, $deserialize:item] => {
        #[derive(Default, Debug, Clone, Copy)]
        pub struct $name(pub $ty);
        impl $name { pub const MAX: $ty = $MAX; }
        impl DataType<'_> for $name { #[inline] $serialize #[inline] $deserialize }
        impl From<$ty> for $name { fn from(num: $ty) -> Self { Self(num) } }
        impl std::ops::Deref for $name {
            type Target = $ty;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    };

}

def!(
    U15(u16),
    MAX: 0x7FFF,
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) {
        let num = self.0;
        // if `num` is less than 128, we can use one byte to store the length.
        if num < 128 {
            (num as u8).serialize(view);
        } else {
            let b1 = 0x80 | (num as u8); // 7 bits with LSB is set.
            let b2 = (num >> 7) as u8; // next 8 bits
            view.write_slice([b1, b2]).unwrap();
        }
    },
    fn deserialize(view: &mut View<&[u8]>) -> Result<Self> {
        let mut num = u8::deserialize(view)? as u16;
        // if LSB is set, read another byte.
        if num >> 7 == 1 {
            num &= 0x7F; // `x` <- get 7 MSB
            let snd = u8::deserialize(view)? as u16;
            num |= snd << 7; // x <- push 8 bits from `x`
        }
        Ok(Self(num))
    }
);

def!(
    U22(u32),
    MAX: 0x3FFFFF,
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) {
        let num = self.0;
        // if `num` is less than 128, we can use one byte to store the length.
        if num < 128 {
            (num as u8).serialize(view);
        } else {
            let b1 = 0x80 | (num as u8); // 7 bits with LSB is set.
            // if `num` is less than 16384, we can use two bytes to store the length.
            if num < 0x4000 {
                let b2 = (num >> 7) as u8; // next 7 bits with LSB is not set.
                view.write_slice([b1, b2]).unwrap();
            } else {
                let b2 = 0x80 | ((num >> 7) as u8); // next 7 bits with LSB is set.
                let b3 = (num >> 14) as u8; // next 8 bits
                view.write_slice([b1, b2, b3]).unwrap();
            }
        }
    },
    fn deserialize(view: &mut View<& [u8]>) -> Result<Self> {
        let mut num = u8::deserialize(view)? as u32;
        // if LSB is set, read another byte.
        if num >> 7 == 1 {
            num &= 0x7F; // `x` <- get 7 MSB

            let snd = u8::deserialize(view)? as u32;
            num |= (snd & 0x7F) << 7; // x <- push 7 MSB from `y`

            if snd >> 7 == 1 {
                let trd = u8::deserialize(view)? as u32;
                num |= trd << 14; // x <- push 8 MSB from `z`
            }
        }
        Ok(Self(num))
    }
);

#[test]
fn test_name() {
    println!("{:x}", 2_u64.pow(21));
}

def!(
    U29(u32),
    MAX: 0x1FFFFFFF,
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) {
        let num = self.0;
        let a1 = (num as u8);
        if num < 128 {
            a1.serialize(view);
        }
        else {
            let a2 = (num >> 7) as u8;
            if num < 0x4000 {
                view.write_slice([0x80 | a1, a2]).unwrap();
            }
            else {
                let a3 = (num >> 14) as u8;
                if num < 0x200000 {
                    view.write_slice([0x80 | a1, 0x80 | a2, a3]).unwrap();
                } else {
                    let a4 = (num >> 21) as u8;
                    view.write_slice([0x80 | a1, 0x80 | a2, 0x80 | a3, a4]).unwrap();
                }
            }
        }
    },
    fn deserialize(view: &mut View<& [u8]>) -> Result<Self> {
        todo!()

        // let mut num = u8::deserialize(view)? as u32;
        // // if LSB is set, read another byte.
        // if num >> 7 == 1 {
        //     num &= 0x7F; // `x` <- get 7 MSB

        //     let snd = u8::deserialize(view)? as u32;
        //     num |= (snd & 0x7F) << 7; // x <- push 7 MSB from `y`

        //     if snd >> 7 == 1 {
        //         let trd = u8::deserialize(view)? as u32;
        //         num |= trd << 14; // x <- push 8 MSB from `z`
        //     }
        // }
        // Ok(Self(num))
    }
);
