use crate::*;

impl DataType<'_> for bool {
    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        u8::serialize(self.into(), view);
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&[u8]>) -> Result<Self> {
        u8::deserialize(view).map(|v| v != 0)
    }
}
impl DataType<'_> for char {
    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        u32::serialize(self.into(), view);
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&[u8]>) -> Result<Self> {
        char::from_u32(u32::deserialize(view)?).ok_or(InvalidChar)
    }
}
macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl DataType<'_> for $rty {
            #[inline]
            fn serialize(self, view: &mut Cursor<impl Bytes>) {
                unsafe {
                    let data = view.data.as_mut();
                    let dst = data.as_mut_ptr().add(view.offset);

                    let total_len = view.offset + size_of::<Self>();
                    view.data.extend_len(total_len, size_of::<Self>());

                    view.offset = total_len;
                    write_num!(self, dst);
                }
            }
            #[inline]
            fn deserialize(view: &mut Cursor<&[u8]>) -> Result<Self> {
                unsafe {
                    let total_len = view.offset + size_of::<Self>();
                    if total_len > view.data.len() { return Err(InsufficientBytes); }
                    
                    let src = view.data.as_ptr().add(view.offset);
                    view.offset = total_len;
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