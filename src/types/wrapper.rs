use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use crate::*;

macro_rules! impls {
    [Encoder for $($name:ty),*] => ($(
        impl<T: Encoder + ?Sized> Encoder for $name {
            #[inline]
            fn encoder(&self, c: &mut impl Write) -> Result<()> { T::encoder(self, c) }
        }
    )*);
    [Decoder for $($name:ident),*] => ($(
        impl<'de, T: Decoder<'de>> Decoder<'de> for $name<T> {
            #[inline]
            fn decoder(c: &mut &'de [u8]) -> Result<Self> { T::decoder(c).map($name::from) }
        }
    )*);
}

impls!(Encoder for &T, &mut T, Box<T>, Rc<T>, Arc<T>);
impls!(Decoder for Box, Rc, Arc, Cell, RefCell);

impl<T> Encoder for std::marker::PhantomData<T> {
    #[inline]
    fn encoder(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

impl<T> Decoder<'_> for std::marker::PhantomData<T> {
    #[inline]
    fn decoder(_: &mut &[u8]) -> Result<Self> {
        Ok(std::marker::PhantomData)
    }
}

impl<T: Encoder + Copy> Encoder for Cell<T> {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        T::encoder(&self.get(), c)
    }
}

impl<T: Encoder> Encoder for RefCell<T> {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        let val = self.try_borrow().map_err(invalid_input)?;
        T::encoder(&val, c)
    }
}

impl Decoder<'_> for Box<str> {
    fn decoder(c: &mut &[u8]) -> Result<Self> {
        String::decoder(c).map(Box::from)
    }
}
impl<'de, T: Decoder<'de>> Decoder<'de> for Box<[T]> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        Vec::decoder(c).map(Box::from)
    }
}