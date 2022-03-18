use crate::*;

macro_rules! impl_data_type_for {
    [$($rty:ty)*] => ($(
        impl DataType<'_> for $rty {
            #[inline]
            fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> { 
                view.write(self)
            }
            #[inline]
            fn deserialize(view: &mut View<&[u8]>) -> Result<Self> { view.read() }
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
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> {
        u8::serialize(self.into(), view)
    }
    #[inline]
    fn deserialize(view: &mut View<&[u8]>) -> Result<Self> {
        u8::deserialize(view).map(|v| v != 0)
    }
}
impl DataType<'_> for char {
    #[inline]
    fn serialize(self, view: &mut View<impl AsMut<[u8]>>) -> Result<()> {
        u32::serialize(self.into(), view)
    }
    #[inline]
    fn deserialize(view: &mut View<&[u8]>) -> Result<Self> {
        char::from_u32(u32::deserialize(view)?).ok_or(InvalidChar)
    }
}
