use crate::*;

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl DataType<'_> for $rty {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { view.write(self).unwrap(); }
            #[inline]
            fn deserialize(view: &mut DataView<&[u8]>) -> Result<Self>{ view.read().ok_or(InsufficientBytes) }
        }
    )*);
}

impl_data_type_for!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
    f32 f64
);

impl DataType<'_> for bool {
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write::<u8>(self.into()).unwrap();
    }
    #[inline]
    fn deserialize(view: &mut DataView<&[u8]>) -> Result<Self> {
        view.read().map(|n: u8| n != 0).ok_or(InsufficientBytes)
    }
}

impl DataType<'_> for char {
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write::<u32>(self.into()).unwrap();
    }
    #[inline]
    fn deserialize(view: &mut DataView<&[u8]>) -> Result<Self> {
        let num: u32 = view.read().ok_or(InsufficientBytes)?;
        char::from_u32(num).ok_or(InvalidChar)
    }
}
