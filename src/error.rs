use std::{str::Utf8Error, string::FromUtf8Error};

pub trait Error: Sized {
    fn insufficient_bytes() -> Self;

    fn invalid_data() -> Self;

    /// Errors which can occur when attempting to interpret a sequence of u8 as a string.
    #[inline]
    fn utf8_err(_: Utf8Error) -> Self {
        Self::invalid_data()
    }

    /// A possible error value when converting a String from a UTF-8 byte vector
    #[inline]
    fn from_utf8_err(err: FromUtf8Error) -> Self {
        Self::utf8_err(err.utf8_error())
    }
}

// =======================================================================

impl Error for () {
    #[inline]
    fn insufficient_bytes() -> Self {
        ()
    }
    #[inline]
    fn invalid_data() -> Self {
        ()
    }    
}