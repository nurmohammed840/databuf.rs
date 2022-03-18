use super::*;

#[derive(Debug, Default)]
pub struct Cursor<T> {
    pub data: T,
    pub offset: usize,
}

impl<T: AsMut<[u8]>> Cursor<T> {
    #[inline]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) -> Result<()> {
        let data = self.data.as_mut();
        let src = slice.as_ref();
        let count = src.len();
        unsafe {
            let dst = data.as_mut_ptr().add(self.offset);
            check_len!(data, self, count);            
            ptr::copy_nonoverlapping(src.as_ptr(), dst, count);
        }
        Ok(())
    }
}

impl<'a> Cursor<&'a [u8]> {
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
    pub fn read_slice(&mut self, len: usize) -> Result<&'a [u8]> {
        let total_len = self.offset + len;
        let slice = self.data.get(self.offset..total_len).ok_or(InsufficientBytes)?;
        self.offset = total_len;
        Ok(slice)
    }
}

impl<T> From<T> for Cursor<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self { data, offset: 0 }
    }
}
