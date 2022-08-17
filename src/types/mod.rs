use super::*;
use len::Len;
use std::io::{Error, ErrorKind};

mod alloc;
mod collection;
mod compound;
mod enumerate;
mod scalar;
mod wrapper;

macro_rules! encode_len {
    [$c: expr, $len: expr] => {
        let len = $len.try_into().unwrap();
        Len::new(len)
            .ok_or(Error::new(ErrorKind::InvalidInput, format!("Max payload length: {}, But got {len}", Len::MAX)))?
            .encoder($c)?;
    }
}
pub(self) use encode_len;

fn invalid_data<E>(error: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::InvalidData, error)
}

fn get_slice<'a>(this: &mut &'a [u8], len: usize) -> Result<&'a [u8]> {
    if len <= this.len() {
        unsafe {
            let slice = this.get_unchecked(..len);
            *this = this.get_unchecked(len..);
            Ok(slice)
        }
    } else {
        Err(Error::new(ErrorKind::UnexpectedEof, "Insufficient bytes"))
    }
}
