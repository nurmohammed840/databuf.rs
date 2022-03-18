use crate::*;
use core::fmt::Debug;

/// This trait contains many unsafe methods for efficiently reading and writing data.
///
/// Those Methods are unsafe because they do not check the index bounds.
///
/// Those methods are safely used by internal. And shouldn't expect to be used by user.
/// You almost never have to implement this trait for your own types.
pub trait Endian: Copy + Default + Debug + PartialEq + PartialOrd {
    unsafe fn write_at_le(self, dst: *mut u8);
    unsafe fn write_at_be(self, dst: *mut u8);
    unsafe fn write_at_ne(self, dst: *mut u8);
    unsafe fn from_bytes_le(src: *const u8) -> Self;
    unsafe fn from_bytes_be(src: *const u8) -> Self;
    unsafe fn from_bytes_ne(src: *const u8) -> Self;
}

macro_rules! impl_endian_for {
    [$($rty:ty : $nbytes:tt)*] => ($(
        impl Endian for $rty {
            #[inline]
            unsafe fn write_at_ne(self, dst: *mut u8) { ptr::copy_nonoverlapping(&self as *const _ as *const u8, dst, $nbytes) }
            #[inline]
            unsafe fn write_at_le(self, dst: *mut u8) { ptr::copy_nonoverlapping(self.to_le_bytes().as_ptr(), dst, $nbytes) }
            #[inline]
            unsafe fn write_at_be(self, dst: *mut u8) { ptr::copy_nonoverlapping(self.to_be_bytes().as_ptr(), dst, $nbytes) }
            #[inline]
            unsafe fn from_bytes_le(src: *const u8) -> Self { Self::from_le_bytes(ptr::read(src as *const [u8; $nbytes])) }
            #[inline]
            unsafe fn from_bytes_be(src: *const u8) -> Self { Self::from_be_bytes(ptr::read(src as *const [u8; $nbytes])) }
            #[inline]
            unsafe fn from_bytes_ne(src: *const u8) -> Self {
                let mut tmp = MaybeUninit::<Self>::uninit();
                ptr::copy_nonoverlapping(src, tmp.as_mut_ptr() as *mut u8, $nbytes);
                tmp.assume_init()
            }
        }
    )*);
}
#[cfg(target_pointer_width = "16")]
impl_endian_for!(usize:2 isize:2);
#[cfg(target_pointer_width = "32")]
impl_endian_for!(usize:4 isize:4);
#[cfg(target_pointer_width = "64")]
impl_endian_for!(usize:8 isize:8);

impl_endian_for!(
    u8:1 u16:2 u32:4 u64:8 u128:16
    i8:1 i16:2 i32:4 i64:8 i128:16
    f32:4 f64:8
);

#[inline]
pub unsafe fn write_num<E: Endian>(src: *const u8) -> E {
    #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
    return E::from_bytes_le(src);
    // ---------------------------------------------
    #[cfg(all(target_endian = "little", feature = "BE"))]
    return E::from_bytes_be(src);
    // ---------------------------------------------
    return E::from_bytes_ne(src);
}

#[inline]
pub unsafe fn read_num<E: Endian>(num: E, dst: *mut u8) {
    #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
    return num.write_at_le(dst);
    // ---------------------------------------------
    #[cfg(all(target_endian = "little", feature = "BE"))]
    return num.write_at_be(dst);
    // ---------------------------------------------
    return num.write_at_ne(dst);
}
