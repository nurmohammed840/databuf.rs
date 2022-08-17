use super::*;
use std::collections::{BTreeMap, BTreeSet};

impl<T: Encoder> Encoder for Vec<T> {
    #[inline]
    fn size_hint(&self) -> usize {
        Len::SIZE + self.iter().map(T::size_hint).sum::<usize>()
    }

    #[inline]
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        encode_len!(c, self.len());
        for item in self {
            item.encoder(c)?;
        }
        Ok(())
    }
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

// ------------------------------------------------------------------------------

impl<K: Encoder, V: Encoder> Encoder for BTreeMap<K, V> {
    fn size_hint(&self) -> usize {
        Len::SIZE
            + self
                .iter()
                .map(|(k, v)| k.size_hint() + v.size_hint())
                .sum::<usize>()
    }

    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        encode_len!(c, self.len());

        for (k, v) in self.iter() {
            k.encoder(c)?;
            v.encoder(c)?;
        }
        Ok(())
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

impl<T: Encoder> Encoder for BTreeSet<T> {
    fn encoder(&self, c: &mut impl Write) -> Result<()> {
        encode_len!(c, self.len());
        for v in self.iter() {
            v.encoder(c)?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree() {
        let map = BTreeMap::from_iter((0u8..255).map(|i| (i, i)));
        assert_eq!(map, BTreeMap::decode(&map.encode()).unwrap());
    }
}
