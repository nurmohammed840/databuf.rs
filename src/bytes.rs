use super::*;

pub trait Bytes {
    /// # Panics
    /// This function may panic, if the data is slice `&mut [u8]`, and has not enough capacity.
    ///
    /// But If the data is vector `Vec<u8>`, then it may reserve extra capacity if necessary.
    fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize;

    /// # Panics
    /// This function may panic. should be used with care.
    #[doc(hidden)]
    unsafe fn extend_len(&mut self, total_len: usize, count: usize);

    fn as_ref(&mut self) -> &[u8];
    fn as_mut(&mut self) -> &mut [u8];
}

macro_rules! impls {
    [$ty:ty : $func:item] => {
        impl Bytes for $ty {
            #[inline]
            fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize {
                let src = slice.as_ref();
                let count = src.len();
                unsafe {
                    let total_len = offset + count;
                    self.extend_len(total_len, count);
                    ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().add(offset), count);
                    total_len
                }
            }
            #[inline] fn as_ref(&mut self) -> &[u8] { self }
            #[inline] fn as_mut(&mut self) -> &mut [u8] { self }
            #[inline]
            #[doc(hidden)]
            $func
        }
    };
}

impls!(&mut Vec<u8>:
    unsafe fn extend_len(&mut self, total_len: usize, count: usize) {
        if total_len > self.len() {
            self.reserve(count);
            self.set_len(total_len);
        }
    }
);
impls!(&mut [u8]:
    unsafe fn extend_len(&mut self, total_len: usize, _: usize) {
        assert!(total_len <= self.len(), "total len: {total_len} <= buffer len: {}", self.len());
    }
);