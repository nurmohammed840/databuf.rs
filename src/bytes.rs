use super::*;

pub trait Bytes {
    /// # Panics
    /// This function may panic, if the data is slice `&mut [u8]`, and has not enough capacity.
    /// 
    /// But If the data is vector `Vec<u8>`, then it may reserve extra capacity if necessary.
    fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize;
}

impl Bytes for Vec<u8> {
    fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize {
        let src = slice.as_ref();
        let count = src.len();
        let total_len = offset + count;
        unsafe {
            if total_len > self.len() {
                self.reserve(count);
                self.set_len(total_len);
            }
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().add(offset), count);
        }
        total_len
    }
}

impl Bytes for [u8] {
    fn write_slice_at(&mut self, offset: usize, slice: impl AsRef<[u8]>) -> usize {
        let src = slice.as_ref();
        let count = src.len();
        let total_len = offset + count;
        if total_len > self.len() {
            panic!("InsufficientBytes");
        }
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().add(offset), count);
        }
        total_len
    }
}