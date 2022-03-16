use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => (
        #[allow(unused_variables)]
        $(
            impl<'de, $($name,)*> DataType<'de> for ($($name,)*)
            where
                $($name: DataType<'de>,)*
            {
                #[inline]
                fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) { $(self.$idx.serialize(view);)* }
                #[inline]
                fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> { Ok(($($name::deserialize(view)?),*)) }
            }
        )*
    );
}

impl_data_type_for_typle!(
    (),
    (A:0, B:1),
    (A:0, B:1, C:2),
    (A:0, B:1, C:2, D:3),
    (A:0, B:1, C:2, D:3, E:4),
    (A:0, B:1, C:2, D:3, E:4, F:5),
    (A:0, B:1, C:2, D:3, E:4, F:5, G:6)
);

impl<'de, T: DataType<'de>, const N: usize> DataType<'de> for [T; N] {
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        for item in self {
            item.serialize(view);
        }
    }
    #[inline]
    fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
        #[cfg(feature = "nightly")]
        return [(); N].try_map(|_| T::deserialize(view));

        #[cfg(not(feature = "nightly"))]
        return (0..N)
            .map(|_| T::deserialize(view))
            .collect::<Result<Vec<_>>>()
            .map(|v| unsafe { v.try_into().unwrap_unchecked() });
    }
}

impl<'de, const N: usize> DataType<'de> for &'de [u8; N] {
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        view.write_slice(self).unwrap()
    }
    #[inline]
    fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
        view.read_slice(N)
            .map(|bytes| unsafe { bytes.try_into().unwrap_unchecked() })
            .ok_or(InsufficientBytes)
    }
}
