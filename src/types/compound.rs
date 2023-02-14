use std::convert::TryFrom;

use crate::*;

macro_rules! impl_data_type_for_typle {
    [$(($($name: ident : $idx: tt),*)),*]  => ($(
        impl<$($name,)*> Encode for ($($name,)*)
        where
            $($name: Encode,)*
        {
            #[inline] fn encode<const CONFIG: u8>(&self, _c: &mut impl Write) -> io::Result<()> {
                $(self.$idx.encode::<CONFIG>(_c)?;)*
                Ok(())
            }
        }
        impl<'de, $($name,)*> Decode<'de> for ($($name,)*)
        where
            $($name: Decode<'de>,)*
        {
            #[inline] fn decode<const CONFIG: u8>(_c: &mut &'de [u8]) -> Result<Self> {
                Ok(($($name::decode::<CONFIG>(_c)?,)*))
            }
        }
    )*);
}

impl_data_type_for_typle!(
    (),
    (T:0),
    (T:0, T1:1),
    (T:0, T1:1, T2:2),
    (T:0, T1:1, T2:2, T3:3),
    (T:0, T1:1, T2:2, T3:3, T4:4),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14),
    (T:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15)
);

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    #[inline]
    fn encode<const CONFIG: u8>(&self, c: &mut impl Write) -> io::Result<()> {
        self.iter().try_for_each(|item| item.encode::<CONFIG>(c))
    }
}

impl<'de, T, const N: usize> Decode<'de> for [T; N]
where
    T: Decode<'de>,
{
    #[inline]
    fn decode<const CONFIG: u8>(cursor: &mut &'de [u8]) -> Result<Self> {
        utils::try_collect::<_, _, CONFIG>(cursor, N)
            .map(|vec: Vec<T>| unsafe { <[T; N]>::try_from(vec).unwrap_unchecked() })
    }
}

impl<'de: 'a, 'a, const N: usize> Decode<'de> for &'a [u8; N] {
    #[inline]
    fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
        // SEAFTY: bytes.len() == N
        utils::get_slice(c, N).map(|bytes| unsafe { <&[u8; N]>::try_from(bytes).unwrap_unchecked() })
    }
}
