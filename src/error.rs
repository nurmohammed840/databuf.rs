use std::error::Error;
use std::fmt::{Display, self};

#[derive(Debug)]
pub struct UnknownDiscriminant {
    pub ident: &'static str,
    pub discriminant: u16,
}

#[derive(Debug)]
pub struct InsufficientBytes;

#[derive(Debug)]
pub struct InvalidChar;

#[derive(Debug)]
pub struct IntegerOverflow;

#[derive(Debug)]
pub struct InvalidBoolValue;

impl Error for UnknownDiscriminant {}
impl Error for InsufficientBytes {}
impl Error for InvalidChar {}
impl Error for IntegerOverflow {}
impl Error for InvalidBoolValue {}

impl Display for UnknownDiscriminant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { ident, discriminant } = self;
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
