use std::{str::Utf8Error, string::FromUtf8Error};

pub trait Error: Sized {
    /// Errors which can occur when not enough bytes are available to read.
    fn insufficient_bytes() -> Self;

    /// Errors which can occur when parsing invalid `char`, 'UTF-8', `length` (for variable-length records)
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

#[derive(Debug, Clone, PartialEq)]
/// Default Internal errors type
pub enum ErrorKind {
    /// invalid `char` or `length` (for variable-length records).
    InvalidData,
    /// not enough bytes to read.
    InsufficientBytes,
    /// Errors which can occur when attempting to interpret a sequence of u8 as a string.
    Utf8Error(Utf8Error),
    /// A possible error value when converting a String from a UTF-8 byte vector.
    FromUtf8Error(FromUtf8Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for ErrorKind {}

impl Error for ErrorKind {
    fn insufficient_bytes() -> Self {
        Self::InsufficientBytes
    }
    fn invalid_data() -> Self {
        Self::InvalidData
    }
    fn utf8_err(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
    fn from_utf8_err(err: FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
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
