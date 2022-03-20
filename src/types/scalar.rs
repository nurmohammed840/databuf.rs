#![allow(unreachable_code)]
use crate::*;

impl DataType<'_> for bool {
    #[inline]
    fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
        u8::serialize(self.into(), view)
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&[u8]>) -> Result<Self> {
        u8::deserialize(view).map(|v| v != 0)
    }
}
impl DataType<'_> for char {
    #[inline]
    fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
        u32::serialize(self.into(), view)
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
            fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
                unsafe {
                    let data = view.data.as_mut();
                    let dst = data.as_mut_ptr().add(view.offset);
                    ret_err_or_add! { (view.offset; + size_of::<Self>()) > data.len() };
                    write_num!(self, dst);
                }
            }
            #[inline]
            fn deserialize(view: &mut Cursor<&[u8]>) -> Result<Self> {
                unsafe {
                    let data = view.data.as_ref();
                    let src = data.as_ptr().add(view.offset);
                    ret_err_or_add!{ (view.offset; + size_of::<Self>()) > data.len() };
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
        return Ok(read_unaligned($src));
    };
}
macro_rules! write_num {
    [$val:tt, $dst: expr] => (
        #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]{
            write_unaligned($val.to_le_bytes().as_ptr() , $dst, size_of::<Self>());
            return Ok(());
        }
        #[cfg(all(target_endian = "little", feature = "BE"))]{
            write_unaligned($val.to_be_bytes().as_ptr() , $dst, size_of::<Self>());
            return Ok(());
        }
        write_unaligned(&$val as *const Self as *const u8, $dst, size_of::<Self>());
        return Ok(());
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