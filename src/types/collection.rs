use super::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

macro_rules! impl_encoder {
    (Set) => {
        #[inline]
        fn encoder(&self, c: &mut impl Write) -> Result<()> {
            encode_len!(c, self.len());
            self.iter().try_for_each(|item| item.encoder(c))
        }
    };
    (Map) => {
        #[inline]
        fn encoder(&self, c: &mut impl Write) -> Result<()> {
            encode_len!(c, self.len());
            for (k, v) in self.iter() {
                k.encoder(c)?;
                v.encoder(c)?;
            }
            Ok(())
        }
    };
}

impl<T: Encoder> Encoder for Vec<T> {
    impl_encoder! {Set}
}

impl<T: Encoder, S> Encoder for HashSet<T, S> {
    impl_encoder! {Set}
}

impl<T: Encoder> Encoder for BTreeSet<T> {
    impl_encoder! {Set}
}

impl<K: Encoder, V: Encoder, S> Encoder for HashMap<K, V, S> {
    impl_encoder! {Map}
}

impl<K: Encoder, V: Encoder> Encoder for BTreeMap<K, V> {
    impl_encoder! {Map}
}

impl<'de, T> Decoder<'de> for Vec<T>
where
    T: Decoder<'de>,
{
    #[inline]
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let len = Len::decoder(c)?.into_inner();
        let mut vec = Vec::with_capacity(len.try_into().unwrap());
        for _ in 0..len {
            vec.push(T::decoder(c)?);
        }
        Ok(vec)
    }
}

impl<'de, T> Decoder<'de> for BTreeSet<T>
where
    T: Decoder<'de> + Ord,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        Vec::<T>::decoder(c).map(BTreeSet::from_iter)
    }
}

impl<'de, K, V> Decoder<'de> for BTreeMap<K, V>
where
    K: Decoder<'de> + Ord,
    V: Decoder<'de>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        Vec::<(K, V)>::decoder(c).map(BTreeMap::from_iter)
    }
}

impl<'de, K, V, S> Decoder<'de> for HashMap<K, V, S>
where
    K: Decoder<'de> + Eq + std::hash::Hash,
    V: Decoder<'de>,
    S: std::hash::BuildHasher + Default,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        Vec::<(K, V)>::decoder(c).map(HashMap::from_iter)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn branch() {
        let map = Vec::from_iter(0..u16::MAX);

        let time = Instant::now();

        for _ in 0..3 {
            let byte = map.encode();
            assert_eq!(131073, byte.len());
        }

        println!("{:?}", time.elapsed());
    }

    // fn hashmap() {
    //     let map = HashMap::<u8, u8>::new();
    //     map.encode();
    // }

    #[test]
    fn btree() {
        let map = BTreeMap::from_iter((0u8..255).map(|i| (i, i)));
        assert_eq!(map, BTreeMap::decode(&map.encode()).unwrap());
    }
}
