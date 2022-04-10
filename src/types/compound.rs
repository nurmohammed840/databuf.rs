use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => (
        $(
            impl<$($name,)*> Encoder for ($($name,)*)
            where
                $($name: Encoder,)*
            {
                const SIZE: usize = 0 $(+$name::SIZE)*;

                #[inline]
                fn size_hint(&self) -> usize { 0 $(+ self.$idx.size_hint())* }

                #[inline]
                fn encoder(self, _c: &mut impl Array<u8>) {
                    $(self.$idx.encoder(_c);)*
                }
            }

            impl<'de, E, $($name,)*> Decoder<'de, E> for ($($name,)*)
            where
                $($name: Decoder<'de, E>,)*
            {
                #[inline]
                fn decoder(_c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
                    Ok(($($name::decoder(_c)?),*))
                }
            }
        )*
    );
}
impl_data_type_for_typle!(
    (),
    (T:0, T2:1),
    (T:0, T2:1, T3:2),
    (T:0, T2:1, T3:2, T4:3),
    (T:0, T2:1, T3:2, T4:3, T5:4),
    (T:0, T2:1, T3:2, T4:3, T5:4, T6:5),
    (T:0, T2:1, T3:2, T4:3, T5:4, T6:5, T7:6)
);

impl<T: Encoder, const N: usize> Encoder for [T; N] {
    const SIZE: usize = N * T::SIZE;
    // const IS_DYNAMIC: bool = T::IS_DYNAMIC;
    #[inline]
    fn size_hint(&self) -> usize {
        self.iter().map(T::size_hint).sum()
    }

    #[inline]
    fn encoder(self, c: &mut impl Array<u8>) {
        for item in self {
            item.encoder(c);
        }
    }
}
impl<'de, E, T: Decoder<'de, E>, const N: usize> Decoder<'de, E> for [T; N] {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        #[cfg(feature = "nightly")]
        return [(); N].try_map(|_| T::decoder(c));

        #[cfg(not(feature = "nightly"))]
        return (0..N)
            .map(|_| T::decoder(c))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| unsafe { v.try_into().unwrap_unchecked() });
    }
}

impl<const N: usize> Encoder for &[u8; N] {
    const SIZE: usize = N;
    // const IS_DYNAMIC: bool = false;
    #[inline]
    fn encoder(self, c: &mut impl Array<u8>) {
        c.extend_from_slice(self);
    }
}

impl<'de, E: Error, const N: usize> Decoder<'de, E> for &'de [u8; N] {
    #[inline]
    fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, E> {
        c.read_slice(N)
            .map(|bytes| unsafe { bytes.try_into().unwrap_unchecked() })
            .ok_or_else(E::insufficient_bytes)
    }
}
