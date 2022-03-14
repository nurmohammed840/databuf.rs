use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => ($(
        impl<$($name,)*> DataType for ($($name,)*)
        where
            $($name: DataType,)*
        {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { $(self.$idx.serialize(view);)* }
            #[inline]
            fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> { Ok(($($name::deserialize(view)?),*)) }
        }
    )*);
}
impl_data_type_for_typle!(
    (),
    (A:0, B:1),
    (A:0, B:1, C:2),
    (A:0, B:1, C:2, D:3),
    (A:0, B:1, C:2, D:3, E:4)
);

impl<T: DataType, const N: usize> DataType for [T; N] {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        for item in self {
            item.serialize(view);
        }
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        #[cfg(feature = "nightly")]
        return [(); N].try_map(|_| T::deserialize(view));

        #[cfg(not(feature = "nightly"))]
        return (0..N)
            .map(|_| T::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|v| unsafe { v.try_into().unwrap_unchecked() });
    }
}
