
// Range of bytes of `U15` are,
// 1. 0..127         represented by 1 byte
// 2. 128..0x7FFF    represented by 2 bytes

// Range of bytes of `U22` are,
//
// 1. 0..127
// 2. 128..0x3FFF
// 3. 0x4000..0x3FFFFF

// Range of bytes of `U29` are,
//
// 1. 0..127
// 2. 128..0x3FFF
// 3. 0x4000..0x1FFFFF
// 4. 0x400000..0x1FFFFFFF

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
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> {
        let num = self.0;
        let b1 = num as u8;
        if num < 128 {
            // No LSB set (`b1`). bcs We checked `num` is less then `128`
            b1.serialize(view)
        } else {
            let b1 = 0x80 | b1; // 7 bits with LSB is set.
            let b2 = (num >> 7) as u8; // next 8 bits
            view.write_slice([b1, b2])
        }
    },
    fn deserialize(view: &mut View<&[u8]>) -> Result<Self> {
        let mut num = u8::deserialize(view)? as u16;
        // if LSB is set, read another byte.
        if num >> 7 == 1 {
            num &= 0x7F; // `x` <- get 7 bits

            let snd = u8::deserialize(view)? as u16;
            num |= snd << 7; // x <- push 8 bits
        }
        Ok(Self(num))
    }
);

def!(
    U22(u32),
    MAX: 0x3FFFFF,
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> {
        let num = self.0;
        let b1 = num as u8;
        if num < 128 {
            // No LSB set (`b1`). bcs We checked `num` is less then `128`
            b1.serialize(view)
        } else {
            let b1 = 0x80 | b1; // set LSB
            let b2 = (num >> 7) as u8; 
            if num < 0x4000 {
                // No LSB set (`b2`). bcs We checked `num` is less then `0x4000`
                view.write_slice([b1, b2])
            } else {
                let b2 = 0x80 | b2; //  set LSB
                let b3 = (num >> 14) as u8;  // read full 8 bits
                view.write_slice([b1, b2, b3])
            }
        }
    },
    fn deserialize(view: &mut View<& [u8]>) -> Result<Self> {
        let mut num = u8::deserialize(view)? as u32;
        // if LSB is set, read another byte.
        if num >> 7 == 1 {
            num &= 0x7F; // x <- get 7 bits

            let snd = u8::deserialize(view)? as u32;
            num |= (snd & 0x7F) << 7; // x <- push 7 bits from `snd`

            if snd >> 7 == 1 {
                let trd = u8::deserialize(view)? as u32;
                num |= trd << 14; // x <- push 8 bits from `trd`
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
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> {
        let num = self.0;
        let b1 = num as u8;
        if num < 128 {
            b1.serialize(view)
        } else {
            let b1 = 0x80 | b1;
            let b2 = (num >> 7) as u8;
            
            if num < 0x4000 {
                view.write_slice([b1, b2])
            } else {
                let b2 = 0x80 | b2;
                let b3 = (num >> 14) as u8;
                
                if num < 0x200000 {
                    view.write_slice([b1, b2, b3])
                } else {
                    let b3 = 0x80 | b3;
                    let b4 = (num >> 21) as u8;
                    view.write_slice([b1, b2, b3, b4])
                }
            }
        }
    },
    fn deserialize(view: &mut View<& [u8]>) -> Result<Self> {
        let mut num = u8::deserialize(view)? as u32;
        // if LSB is set, read another byte.
        if num >> 7 == 1 {
            num &= 0x7F; 

            let snd = u8::deserialize(view)? as u32;
            num |= (snd & 0x7F) << 7; 

            if snd >> 7 == 1 {
                let trd = u8::deserialize(view)? as u32;
                num |= (trd & 0x7F) << 14;

                if trd >> 7 == 1 {
                    let fth = u8::deserialize(view)? as u32;
                    num |= fth << 21;
                }
            }
        }
        Ok(Self(num))
    }
);
