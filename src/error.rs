use thiserror::Error;

#[derive(Error, Debug)]
#[error("unknown `{discriminant}` discriminator of `{ident}`")]
pub struct UnknownDiscriminant {
    pub ident: &'static str,
    pub discriminant: u16,
}

#[derive(Error, Debug)]
#[error("insufficient bytes")]
pub struct InsufficientBytes;

#[derive(Error, Debug)]
#[error("invalid char")]
pub struct InvalidChar;

#[derive(Error, Debug)]
#[error("out of range integral type conversion attempted")]
pub struct IntegerOverflow;
