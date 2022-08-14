use crate::*;

// macro_rules! impl_data_type_for_typle {
//     [$(($($name: ident : $idx: tt),*)),*]  => (
//         $(
//             impl<$($name,)*> Encoder for ($($name,)*)
//             where
//                 $($name: Encoder,)*
//             {
//                 const SIZE: usize = 0 $(+$name::SIZE)*;

//                 #[inline]
//                 fn size_hint(&self) -> usize { 0 $(+ self.$idx.size_hint())* }

//                 #[inline]
//                 fn encoder(&self, _c: &mut impl Array<u8>) {
//                     $(self.$idx.encoder(_c);)*
//                 }
//             }

//             impl<'de, $($name,)*> Decoder<'de> for ($($name,)*)
//             where
//                 $($name: Decoder<'de>,)*
//             {
//                 #[inline]
//                 fn decoder(_c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
//                     Ok(($($name::decoder(_c)?),*))
//                 }
//             }
//         )*
//     );
// }
// impl_data_type_for_typle!(
//     (),
//     (T:0, T2:1),
//     (T:0, T2:1, T3:2),
//     (T:0, T2:1, T3:2, T4:3),
//     (T:0, T2:1, T3:2, T4:3, T5:4),
//     (T:0, T2:1, T3:2, T4:3, T5:4, T6:5),
//     (T:0, T2:1, T3:2, T4:3, T5:4, T6:5, T7:6)
// );

// impl<T: Encoder, const N: usize> Encoder for [T; N] {
//     const SIZE: usize = N * T::SIZE;

//     #[inline]
//     fn size_hint(&self) -> usize {
//         self.iter().map(T::size_hint).sum()
//     }

//     #[inline]
//     fn encoder(&self, c: &mut impl Array<u8>) {
//         for item in self {
//             item.encoder(c);
//         }
//     }
// }
// impl<'de, T: Decoder<'de>, const N: usize> Decoder<'de> for [T; N] {
//     #[inline]
//     fn decoder(c: &mut Cursor<&'de [u8]>) -> Result<Self, &'static str> {
//         #[cfg(feature = "nightly")]
//         return [(); N].try_map(|_| T::decoder(c));

//         #[cfg(not(feature = "nightly"))]
//         return (0..N)
//             .map(|_| T::decoder(c))
//             .collect::<Result<Vec<_>, _>>()
//             .map(|v| unsafe { v.try_into().unwrap_unchecked() });
//     }
// }

impl<const N: usize> Encoder for &[u8; N] {
    const SIZE: usize = N;

    #[inline]
    fn encoder(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_all(self.as_slice())
    }
}

impl<'de, const N: usize> Decoder<'de> for &'de [u8; N] {
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        // SEAFTY: bytes.len() == N
        get_slice(c, N).map(|bytes| unsafe { bytes.try_into().unwrap_unchecked() })
    }
}
