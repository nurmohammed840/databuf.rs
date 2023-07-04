use std::error::Error;
use std::fmt::{self, Display};

/// `enum` uses a discriminator to distinguish its variants.
///
/// This `UnknownDiscriminant` can happen when decoding an `enum` type that has an unknown discriminator value.
#[derive(Debug)]
pub struct UnknownDiscriminant {
    /// Path of the `enum` struct
    pub ident: &'static str,
    /// Unrecognized discriminant value
    pub discriminant: u16,
}

/// Occurs when there are not enough bytes in the input buffer to complete the decoding process.
#[derive(Debug)]
pub struct InsufficientBytes;

/// Occurs when invalid utf8 character found during the decoding process.
#[derive(Debug)]
pub struct InvalidChar;

/// Occurs when the integer value exceeds the maximum value that can be represented by the target integer type.
#[derive(Debug)]
pub struct IntegerOverflow;

/// Occurs during decoding when a [bool] value is expected, but the byte contains a value that is not `0` or `1`.
#[derive(Debug)]
pub struct InvalidBoolValue;

impl Error for UnknownDiscriminant {}
impl Error for InsufficientBytes {}
impl Error for InvalidChar {}
impl Error for IntegerOverflow {}
impl Error for InvalidBoolValue {}

impl Display for UnknownDiscriminant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ident,
            discriminant,
        } = self;
        writeln!(f, "unknown `{discriminant}` discriminator of `{ident}`")
    }
}
impl Display for InsufficientBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "insufficient bytes")
    }
}
impl Display for InvalidChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "invalid char")
    }
}
impl Display for IntegerOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "out of range integral type conversion attempted")
    }
}
impl Display for InvalidBoolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "invalid value for bool type: expected 0 or 1")
    }
}
