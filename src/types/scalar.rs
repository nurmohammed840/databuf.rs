use crate::*;

impl Encoder for bool {
    #[inline]
    fn encoder(&self, c: &mut impl Array<u8>) {
        c.push(*self as u8);
    }
}
impl Decoder<'_> for bool {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, &'static str> {
        u8::decoder(c).map(|b| b != 0)
    }
}

impl Encoder for char {
    #[inline]
    fn encoder(&self, c: &mut impl Array<u8>) {
        u32::encoder(&u32::from(*self), c);
    }
}
impl Decoder<'_> for char {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, &'static str> {
        char::from_u32(u32::decoder(c)?).ok_or("Invalid char")
    }
}

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl Encoder for $rty {
            #[inline]
            fn encoder(&self, c: &mut impl Array<u8>) {
                let len = c.len();
                let total_len = len + size_of::<Self>();
                c.ensure_capacity(total_len);
                unsafe {
                    write_num!(self, c.as_mut_ptr().add(len));
                    c.set_len(total_len);
                }
            }
        }
        impl Decoder<'_> for $rty {
            #[inline]
            fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, &'static str> {
                unsafe {
                    let total_len = c.offset + size_of::<Self>();
                    if total_len > c.data.len() { return Err("Insufficient bytes"); }

                    let src = c.data.as_ptr().add(c.offset);
                    c.offset = total_len;
                    read_num!(src);
                };
            }
        }
    )*);
}
macro_rules! read_num {
    [$src: expr] => {
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        return Ok(Self::from_le_bytes(read_unaligned($src)));
        #[cfg(feature = "BE")]
        return Ok(Self::from_be_bytes(read_unaligned($src)));
        #[cfg(feature = "NE")]
        return Ok(read_unaligned($src));
    };
}
macro_rules! write_num {
    [$val:tt, $dst: expr] => (
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        write_unaligned($val.to_le_bytes().as_ptr() , $dst, size_of::<Self>());
        #[cfg(feature = "BE")]
        write_unaligned($val.to_be_bytes().as_ptr() , $dst, size_of::<Self>());
        #[cfg(feature = "NE")]
        write_unaligned($val as *const Self as *const u8, $dst, size_of::<Self>());
    )
}
impl_data_type_for!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
    f32 f64
);

//---------------------------------------------------------------------------------

unsafe fn read_unaligned<T>(src: *const u8) -> T {
    let mut tmp = MaybeUninit::<T>::uninit();
    ptr::copy_nonoverlapping(src, tmp.as_mut_ptr() as *mut u8, size_of::<T>());
    tmp.assume_init()
}
unsafe fn write_unaligned(src: *const u8, dst: *mut u8, count: usize) {
    ptr::copy_nonoverlapping(src, dst, count);
}

//---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scaler_type() {
        for word in [0x_A5C11, 0x_C0DE, 0x_DEC0DE, 0x_ADDED, 0x_AB0DE, 0x_CAFE] {
            assert_eq!(word, u32::decode(&word.encode()).unwrap());
        }
        for word in [
            0x_DEAD_BEEF,
            0x_Faded_Face,
            0x_BAD_F00D,
            0x_C01D_C0FFEE,
            0x_C0CA_C01A,
        ] {
            assert_eq!(word, u64::decode(&word.encode()).unwrap());
        }
    }
}