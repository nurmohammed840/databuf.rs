mod endian;

use core::mem::{size_of, MaybeUninit};
use core::ptr;
pub(crate) use endian::Endian;
use endian::*;

/// This struct represents a data view for reading and writing data in a byte array.
/// When read/write, This increment current offset by the size of the value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct View<T> {
    pub data: T,
    pub offset: usize,
}

impl<'a> View<&'a [u8]> {
    /// Returns remaining slice from the current offset.
    /// It doesn't change the offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([1, 2].as_ref());
    ///
    /// assert_eq!(view.remaining_slice(), &[1, 2]);
    /// view.offset = 42;
    /// assert!(view.remaining_slice().is_empty());
    /// ```
    #[inline]
    pub fn remaining_slice(&self) -> &'a [u8] {
        unsafe { self.data.get_unchecked(self.offset.min(self.data.len())..) }
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([1, 2, 3].as_ref());
    ///
    /// assert_eq!(view.read_slice(2), Some([1, 2].as_ref()));
    /// assert_eq!(view.read_slice(3), None);
    /// ```
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> Option<&'a [u8]> {
        let total_len = self.offset + len;
        let slice = self.data.get(self.offset..total_len)?;
        self.offset = total_len;
        Some(slice)
    }
}

impl<T: AsRef<[u8]>> View<T> {
    /// Reads a value of type `E: Endian` from the View.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([0; 4]);
    ///
    /// view.write::<u16>(42);
    /// view.offset = 0;
    /// assert_eq!(view.read::<u16>(), Some(42));
    /// assert_eq!(view.read::<u32>(), None);
    /// ```
    #[inline]
    pub fn read<E: Endian>(&mut self) -> Option<E> {
        let data = self.data.as_ref();
        let total_len = self.offset + size_of::<E>();
        if total_len > data.len() {
            return None;
        }
        let num = unsafe { write_num(data.as_ptr().add(self.offset)) };
        self.offset = total_len;
        Some(num)
    }

    /// Create a buffer and returns it, from the current offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([1, 2, 3]);
    ///
    /// assert_eq!(view.read_buf(), Some([1, 2]));
    /// assert_eq!(view.read_buf::<3>(), None);
    /// ```
    #[inline]
    pub fn read_buf<const N: usize>(&mut self) -> Option<[u8; N]> {
        let data = self.data.as_ref();
        let total_len = self.offset + N;
        if total_len > data.len() {
            return None;
        }
        unsafe {
            let mut tmp = MaybeUninit::<[u8; N]>::uninit();
            ptr::copy_nonoverlapping(
                data.as_ptr().add(self.offset),
                tmp.as_mut_ptr() as *mut u8,
                N,
            );
            self.offset = total_len;
            Some(tmp.assume_init())
        }
    }
}

impl<T: AsMut<[u8]>> View<T> {
    /// Writes a value of type `E` to the data view. where `E` is a type that implements `Endian`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([0; 3]);
    ///
    /// assert_eq!(view.write(42_u16), Ok(()));
    /// assert_eq!(view.write(123_u32), Err(()));
    /// ```
    #[inline]
    pub fn write<E: Endian>(&mut self, num: E) -> Result<(), ()> {
        let data = self.data.as_mut();
        let total_len = self.offset + size_of::<E>();
        if total_len > data.len() {
            return Err(());
        }
        unsafe { read_num(num, data.as_mut_ptr().add(self.offset)) };
        self.offset = total_len;
        Ok(())
    }

    /// Writes a slice into the data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let mut view = View::new([0; 3]);
    ///
    /// assert_eq!(view.write_slice([4, 2]), Ok(()));
    /// assert_eq!(view.write_slice([1, 2, 3]), Err(()));
    /// assert_eq!(view.data, [4, 2, 0]);
    /// ```
    #[inline]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) -> Result<(), ()> {
        let src = slice.as_ref();
        let data = self.data.as_mut();
        let count = src.len();
        let total_len = self.offset + count;
        if total_len > data.len() {
            return Err(());
        }
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), data.as_mut_ptr().add(self.offset), count);
        }
        self.offset = total_len;
        Ok(())
    }
}

impl<T> View<T> {
    /// # Examples
    ///
    /// ```
    /// use bin_layout::View;
    ///
    /// let view = View::new([0; 16]);
    /// ```
    #[inline]
    pub const fn new(data: T) -> Self {
        Self { data, offset: 0 }
    }
}

impl<T> From<T> for View<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
