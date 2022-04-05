use super::*;

pub trait Bytes {
    /// # Panics
    /// This function may panic, if the data is slice `&mut [u8]`, and has not enough capacity.
    ///
    /// But If the data is vector `Vec<u8>`, then it may reserve extra capacity if necessary.
    fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize;

    // /// # Warning
    // /// #### This function is very very very unsafe!!! Should be used with care.
    // #[doc(hidden)]
    // unsafe fn new_len(&mut self, total_len: usize, count: usize);

    /// # Panic
    /// Calling this method on slice will always panic.
    #[doc(hidden)]
    fn reserve(&mut self, additional: usize);

    /// # Panic
    /// Calling this method on slice will always panic.
    #[doc(hidden)]
    unsafe fn set_len(&mut self, new_len: usize);

    fn as_ref(&mut self) -> &[u8];
    fn as_mut(&mut self) -> &mut [u8];
}

macro_rules! impls {
    [$($ty:ty),* : $reserve:item, $set_len:item] => {$(
        impl Bytes for $ty {
            #[inline]
            fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize {
                let src = slice.as_ref();
                let count = src.len();
                unsafe {
                    let total_len = offset + count;
                    if total_len > self.len() {
                        self.reserve(count);
                        #[allow(clippy::uninit_vec)]
                        self.set_len(total_len);
                    }
                    ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().add(offset), count);
                    total_len
                }
            }
            #[inline] fn as_ref(&mut self) -> &[u8] { self }
            #[inline] fn as_mut(&mut self) -> &mut [u8] { self }

            #[inline]
            #[doc(hidden)]
            $reserve

            #[inline]
            #[doc(hidden)]
            $set_len
        }
    )*};
}

impls!(&mut Vec<u8>, Vec<u8>:
    fn reserve(&mut self, additional: usize) { Vec::reserve( self, additional) },
    unsafe fn set_len(&mut self, new_len: usize) { Vec::set_len(self, new_len) }
);

impls!(&mut [u8]:
    fn reserve(&mut self, _: usize) { panic!("capacity overflow"); },
    unsafe fn set_len(&mut self, new_len: usize) {
        let len = self.len();
        assert!(new_len <= len, "total len: {new_len} <= buffer len: {len}");
    }
);
