use crate::*;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

macro_rules! impls {
    [Encoder for $($name:ty),*] => ($(
        impl<T: Encoder + ?Sized> Encoder for $name {
            #[inline]
            fn encoder(&self, c: &mut impl Write) -> io::Result<()> { (**self).encoder(c) }
        }
    )*);
    [Decoder for $($name:ident),*] => ($(
        impl<'de, T: Decoder<'de>> Decoder<'de> for $name<T> {
            #[inline]
            fn decoder(c: &mut &'de [u8]) -> Result<Self> { T::decoder(c).map(Self::from) }
        }
    )*);
}

impls!(Encoder for &T, &mut T, Box<T>, Rc<T>, Arc<T>);
impls!(Decoder for Box, Rc, Arc, Cell, RefCell);

macro_rules! impl_sp {
    [$($name: ident),*] => ($(
        impl<'de> Decoder<'de> for $name<str> {
            #[inline] fn decoder(c: &mut &'de [u8]) -> Result<Self> { <&'de str>::decoder(c).map(Self::from) }
        }
        impl<'de, T: Decoder<'de>> Decoder<'de> for $name<[T]> {
            #[inline] fn decoder(c: &mut &'de [u8]) -> Result<Self> { Vec::<T>::decoder(c).map(Self::from) }
        }
    )*);
}
impl_sp!(Box, Rc, Arc);

impl<T> Encoder for std::marker::PhantomData<T> {
    #[inline]
    fn encoder(&self, _: &mut impl Write) -> io::Result<()> {
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
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        self.get().encoder(c)
    }
}

impl<T: Encoder> Encoder for RefCell<T> {
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        self.try_borrow().map_err(invalid_input)?.encoder(c)
    }
}

impl<'a, T> Encoder for Cow<'a, T>
where
    T: ?Sized + Encoder + ToOwned,
{
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        (**self).encoder(c)
    }
}

impl<'de, 'a, T: ?Sized> Decoder<'de> for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Decoder<'de>,
{
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        T::Owned::decoder(c).map(Cow::Owned)
    }
}
