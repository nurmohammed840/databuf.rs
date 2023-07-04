use super::*;
use std::{
    collections::*,
    hash::{BuildHasher, Hash},
};

macro_rules! impl_v2 {
    [Encode for $name: ty where $($ty: tt)*] => {
        impl<$($ty)*> Encode for $name { impl_v2! {@EncoderBody} }
    };
    [Decode for $name: ty where $($ty: tt)*] => {
        impl<'de, $($ty)*> Decode<'de> for $name { impl_v2! {@DecoderBody} }
    };
    [@EncoderBody] => {
        fn encode<const CONFIG: u8>(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
            encode_len!(self, c);
            self.iter().try_for_each(|item| item.encode::<CONFIG>(c))
        }
    };
    [@DecoderBody] => {
        fn decode<const CONFIG: u8>(c: &mut &'de [u8]) -> Result<Self> {
            let len = decode_len!(c);
            utils::try_collect::<_, _, CONFIG>(c, len)
        }
    };
}

impl<T: Encode> Encode for [T] {
    impl_v2! {@EncoderBody}
}

impl_v2!(Encode for Vec<T>             where T: Encode);
impl_v2!(Encode for VecDeque<T>        where T: Encode);
impl_v2!(Encode for LinkedList<T>      where T: Encode);
impl_v2!(Encode for BinaryHeap<T>      where T: Encode);
impl_v2!(Encode for BTreeSet<T>        where T: Encode);
impl_v2!(Encode for BTreeMap<K, V>     where K: Encode, V: Encode);
impl_v2!(Encode for HashSet<T, S>      where T: Encode, S);
impl_v2!(Encode for HashMap<K, V, S>   where K: Encode, V: Encode, S);

impl_v2!(Decode for Vec<T>             where T: Decode<'de>);
impl_v2!(Decode for VecDeque<T>        where T: Decode<'de>);
impl_v2!(Decode for LinkedList<T>      where T: Decode<'de>);
impl_v2!(Decode for BinaryHeap<T>      where T: Decode<'de> + Ord);
impl_v2!(Decode for BTreeSet<T>        where T: Decode<'de> + Ord);
impl_v2!(Decode for BTreeMap<K, V>     where K: Decode<'de> + Ord, V: Decode<'de>);
impl_v2!(Decode for HashSet<T, S>      where T: Decode<'de> + Eq + Hash, S: BuildHasher + Default);
impl_v2!(Decode for HashMap<K, V, S>   where K: Decode<'de> + Eq + Hash, V: Decode<'de>, S: BuildHasher + Default);
