use super::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => (
        $(
            #[cfg(feature = "sizehint")]
            impl<$($name,)*> SizeHint for ($($name,)*)
            where
                $($name: SizeHint,)*
            {
                #[inline] fn size_hint(&self) -> usize { 0 $(+ self.$idx.size_hint())* }
            }

            impl<$($name,)*> Encoder for ($($name,)*)
            where
                $($name: Encoder,)*
            {
                #[inline] fn encoder(&self, _c: &mut impl Write) -> Result<()> {
                    $(self.$idx.encoder(_c)?;)*
                    Ok(())
                }
            }

            impl<'de, $($name,)*> Decoder<'de> for ($($name,)*)
            where
                $($name: Decoder<'de>,)*
            {
                #[inline] fn decoder(_c: &mut &'de [u8]) -> Result<Self> {
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
    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        self.iter().try_for_each(|item| item.encoder(c))
    }
}

impl<'de, T, const N: usize> Decoder<'de> for [T; N]
where
    T: Decoder<'de>,
{
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        #[cfg(feature = "nightly")]
        return [(); N].try_map(|_| T::decoder(c));

        #[cfg(not(feature = "nightly"))]
        return (0..N)
            .map(|_| T::decoder(c))
            .collect::<Result<Vec<_>>>()
            .map(|v| unsafe { v.try_into().unwrap_unchecked() });
    }
}
