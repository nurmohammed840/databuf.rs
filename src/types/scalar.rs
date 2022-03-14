use crate::*;

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl DataType for $rty {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { view.write(self).unwrap(); }
            #[inline]
            fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self>{ Ok(map!(@opt view.read(); InsufficientBytes)) }
        }
    )*);
}

impl_data_type_for!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
    f32 f64
);

impl DataType for bool {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write::<u8>(self.into()).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let num: u8 = map!(@opt view.read(); InsufficientBytes);
        Ok(num != 0)
    }
}

impl DataType for char {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write::<u32>(self.into()).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let num: u32 = map!(@opt view.read(); InsufficientBytes);
        Ok(map!(@opt char::from_u32(num); InvalidData))
    }
}
