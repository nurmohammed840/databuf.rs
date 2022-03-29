use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => (
        #[allow(unused_variables)]
        $(
            impl<'de, $($name,)*> DataType<'de> for ($($name,)*)
            where
                $($name: DataType<'de>,)*
            {
                const SIZE: usize = 0 $(+$name::SIZE)*;
                const IS_DYNAMIC: bool = true $(&& $name::IS_DYNAMIC)*;

                #[inline]
                fn size_hint(&self) -> usize { 0 $(+ self.$idx.size_hint())* }

                #[inline]
                fn serialize(self, view: &mut Cursor<impl Bytes>) { 
                    $(self.$idx.serialize(view);)*
                }
                #[inline]
                fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> { 
                    Ok(($($name::deserialize(view)?),*)) 
                }
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
    const SIZE: usize = N * T::SIZE;
    const IS_DYNAMIC: bool = T::IS_DYNAMIC;

    #[inline]
    fn size_hint(&self) -> usize {
        self.iter().map(T::size_hint).sum() 
    }

    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        for item in self {
            item.serialize(view);
        }
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
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
    const SIZE: usize = N;
    const IS_DYNAMIC: bool = false;
    
    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        view.write_slice(self);
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        view.read_slice(N)
            .map(|bytes| unsafe { bytes.try_into().unwrap_unchecked() })
    }
}
