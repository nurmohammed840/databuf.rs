use crate::*;
use std::{
    collections::*,
    hash::{BuildHasher, Hash},
};

macro_rules! impl_v2 {
    [Encoder for $name: ty where $($ty: tt)*] => {
        impl<$($ty)*> Encoder for $name { impl_v2! {@EncoderBody} }
        impl<Len: LenType, $($ty)*> Encoder for Record<Len, $name> {
            impl_v2! {@EncoderBody}
        }
    };
    [Decoder for $name: ty where $($ty: tt)*] => {
        impl<'de, $($ty)*> Decoder<'de> for $name { impl_v2! {@DecoderBody} }
        impl<'de, Len: LenType,  $($ty)*> Decoder<'de> for Record<Len, $name> {
            impl_v2! {@DecoderBody}
        }
    };
    [@EncoderBody] => {
        #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> {
            encode_len!(self, c);
            self.iter().try_for_each(|item| item.encoder(c))
        }
    };
    [@DecoderBody] => {
        #[inline] fn decoder(c: &mut &'de [u8]) -> Result<Self> {
            let len = decode_len!(c);
            try_collect(c, len)
        }
    };
}

impl<T: Encoder> Encoder for [T] {
    impl_v2! {@EncoderBody}
}

impl<Len: LenType, T: Encoder> Encoder for Record<Len, &[T]> {
    impl_v2! {@EncoderBody}
}

impl_v2!(Encoder for Vec<T>             where T: Encoder);
impl_v2!(Encoder for VecDeque<T>        where T: Encoder);
impl_v2!(Encoder for LinkedList<T>      where T: Encoder);
impl_v2!(Encoder for BinaryHeap<T>      where T: Encoder);
impl_v2!(Encoder for BTreeSet<T>        where T: Encoder);
impl_v2!(Encoder for BTreeMap<K, V>     where K: Encoder, V: Encoder);
impl_v2!(Encoder for HashSet<T, S>      where T: Encoder, S);
impl_v2!(Encoder for HashMap<K, V, S>   where K: Encoder, V: Encoder, S);

impl_v2!(Decoder for Vec<T>             where T: Decoder<'de>);
impl_v2!(Decoder for VecDeque<T>        where T: Decoder<'de>);
impl_v2!(Decoder for LinkedList<T>      where T: Decoder<'de>);
impl_v2!(Decoder for BinaryHeap<T>      where T: Decoder<'de> + Ord);
impl_v2!(Decoder for BTreeSet<T>        where T: Decoder<'de> + Ord);
impl_v2!(Decoder for BTreeMap<K, V>     where K: Decoder<'de> + Ord, V: Decoder<'de>);
impl_v2!(Decoder for HashSet<T, S>      where T: Decoder<'de> + Eq + Hash, S: BuildHasher + Default);
impl_v2!(Decoder for HashMap<K, V, S>   where K: Decoder<'de> + Eq + Hash, V: Decoder<'de>, S: BuildHasher + Default);