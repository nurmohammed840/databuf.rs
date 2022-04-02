use super::*;

#[derive(Debug, Default)]
pub struct Cursor<T> {
    pub data: T,
    pub offset: usize,
}

impl<T: Bytes> Cursor<T> {
    /// Writes a slice into the data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::{Cursor, ErrorKind};
    ///
    /// let mut view = Cursor::new(vec![0; 3]);
    ///
    /// view.write_slice([4, 2]);
    /// assert_eq!(view.offset, 2);
    /// ```
    ///
    /// # Panics
    /// This function may panic, if the data is slice `&mut [u8]`, and has not enough capacity.
    ///
    /// But If the data is vector `Vec<u8>`, then it may reserve extra capacity if necessary.
    #[inline]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) {
        self.offset = self.data.write_slice_at(self.offset, slice);
    }
}

impl<'de> Cursor<&'de [u8]> {
    /// Returns remaining slice from the current offset.
    /// It doesn't change the offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use bin_layout::Cursor;
    ///
    /// let mut view = Cursor::new([1, 2].as_ref());
    ///
    /// assert_eq!(view.remaining_slice(), &[1, 2]);
    /// view.offset = 42;
    /// assert!(view.remaining_slice().is_empty());
    /// ```
    #[inline]
    pub fn remaining_slice(&self) -> &'de [u8] {
        unsafe { self.data.get_unchecked(self.offset.min(self.data.len())..) }
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use bin_layout::{Cursor, ErrorKind};
    /// let mut view = Cursor::new([1, 2, 3].as_ref());
    ///
    /// let slice: Result<_, ()> = view.read_slice(2);
    /// assert_eq!(slice, Ok([1, 2].as_ref()));
    /// 
    /// let slice: Result<_, ErrorKind> = view.read_slice(3);
    /// assert_eq!(slice, Err(ErrorKind::InsufficientBytes));
    /// ```
    #[inline]
    pub fn read_slice<E: Error>(&mut self, len: usize) -> Result<&'de [u8], E> {
        let total_len = self.offset + len;
        let slice = self
            .data
            .get(self.offset..total_len)
            .ok_or_else(E::insufficient_bytes)?;

        self.offset = total_len;
        Ok(slice)
    }
}

impl<T> Cursor<T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self { data, offset: 0 }
    }
}

impl<T> From<T> for Cursor<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
