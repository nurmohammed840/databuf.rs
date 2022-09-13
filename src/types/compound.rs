use std::convert::TryFrom;

use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => (
        $(
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

impl<'de, T: Decoder<'de>, const N: usize> Decoder<'de> for [T; N] {
    #[inline]
    fn decoder(cursor: &mut &'de [u8]) -> Result<Self> {
        try_collect(cursor, N)
            .map(|vec: Vec<T>| unsafe { <[T; N]>::try_from(vec).unwrap_unchecked() })
    }
}

impl<'de: 'a, 'a, const N: usize> Decoder<'de> for &'a [u8; N] {
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        // SEAFTY: bytes.len() == N
        get_slice(c, N).map(|bytes| unsafe { <&[u8; N]>::try_from(bytes).unwrap_unchecked() })
    }
}
