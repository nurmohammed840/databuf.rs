use crate::*;

impl Encoder for bool {
    #[inline]
    fn encoder(self, c: &mut Cursor<impl Bytes>) {
        u8::encoder(self.into(), c);
    }
}
impl<E: Error> Decoder<'_, E> for bool {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
        u8::decoder(c).map(|b| b != 0)
    }
}

impl Encoder for char {
    #[inline]
    fn encoder(self, c: &mut Cursor<impl Bytes>) {
        u32::encoder(self.into(), c);
    }
}
impl<E: Error> Decoder<'_, E> for char {
    #[inline]
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
        char::from_u32(u32::decoder(c)?).ok_or_else(E::invalid_data)
    }
}

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl Encoder for $rty {
            // const IS_DYNAMIC: bool = false;
            #[inline]
            fn encoder(self, c: &mut Cursor<impl Bytes>) {
                unsafe {
                    let data = c.data.as_mut();
                    let dst = data.as_mut_ptr().add(c.offset);

                    let total_len = c.offset + size_of::<Self>();
                    if total_len > c.data.as_ref().len() {
                        c.data.reserve(size_of::<Self>());
                        #[allow(clippy::uninit_vec)]
                        c.data.set_len(total_len);
                    }
                    c.offset = total_len;
                    write_num!(self, dst);
                }
            }
        }
        impl<E: Error> Decoder<'_, E> for $rty {
            #[inline]
            fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
                unsafe {
                    let total_len = c.offset + size_of::<Self>();
                    if total_len > c.data.len() { return Err(E::insufficient_bytes()); }

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
        #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
        return Ok(Self::from_le_bytes(read_unaligned($src)));
        #[cfg(all(target_endian = "little", feature = "BE"))]
        return Ok(Self::from_be_bytes(read_unaligned($src)));
        #[cfg(any(
            feature = "NE",
            all(target_endian = "big", feature = "BE"),
            all(target_endian = "little", not(any(feature = "BE", feature = "NE"))),
        ))]
        return Ok(read_unaligned($src));
    };
}
macro_rules! write_num {
    [$val:tt, $dst: expr] => (
        #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
        write_unaligned($val.to_le_bytes().as_ptr() , $dst, size_of::<Self>());
        #[cfg(all(target_endian = "little", feature = "BE"))]
        write_unaligned($val.to_be_bytes().as_ptr() , $dst, size_of::<Self>());
        #[cfg(any(
            feature = "NE",
            all(target_endian = "big", feature = "BE"),
            all(target_endian = "little", not(any(feature = "BE", feature = "NE"))),
        ))]
        write_unaligned(&$val as *const Self as *const u8, $dst, size_of::<Self>());
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
