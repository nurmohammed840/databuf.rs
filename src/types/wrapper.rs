use crate::*;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

macro_rules! impls {
    [Encode for $($name:ty),*] => ($(
        impl<T: Encode + ?Sized> Encode for $name {
            #[inline]
            fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> { (**self).encode::<CONFIG>(c) }
        }
    )*);
    [Decode for $($name:ident),*] => ($(
        impl<'de, T: Decode<'de>> Decode<'de> for $name<T> {
            #[inline]
            fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> { T::decode::<CONFIG>(c).map(Self::from) }
        }
    )*);
}

impls!(Encode for &T, &mut T, Box<T>, Rc<T>, Arc<T>);
impls!(Decode for Box, Rc, Arc, Cell, RefCell);

macro_rules! impl_sp {
    [$($name: ident),*] => ($(
        impl<'de> Decode<'de> for $name<str> {
            #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
                <&'de str>::decode::<CONFIG>(c).map(Self::from)
            }
        }
        impl<'de, T: Decode<'de>> Decode<'de> for $name<[T]> {
            #[inline] fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
                Vec::<T>::decode::<CONFIG>(c).map(Self::from)
            }
        }
    )*);
}
impl_sp!(Box, Rc, Arc);

impl<T> Encode for std::marker::PhantomData<T> {
    #[inline]
    fn encode<const CONFIG: u8>(&self, _: &mut impl Write) -> io::Result<()> {
        Ok(())
    }
}

impl<T> Decode<'_> for std::marker::PhantomData<T> {
    #[inline]
    fn decode<const CONFIG: u8>(_: &mut &[u8]) -> Result<Self> {
        Ok(std::marker::PhantomData)
    }
}

impl<T: Encode + Copy> Encode for Cell<T> {
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        self.get().encode::<CONFIG>(c)
    }
}

impl<T: Encode> Encode for RefCell<T> {
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        self.try_borrow()
            .map_err(utils::invalid_input)?
            .encode::<CONFIG>(c)
    }
}

impl<'a, T> Encode for Cow<'a, T>
where
    T: ?Sized + Encode + ToOwned,
{
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        (**self).encode::<CONFIG>(c)
    }
}

impl<'de, 'a, T: ?Sized> Decode<'de> for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Decode<'de>,
{
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        T::Owned::decode::<CONFIG>(c).map(Cow::Owned)
    }
}
