use super::*;
use std::collections::BTreeMap;

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
    K: Decoder<'de>,
    V: Decoder<'de>,
{
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let len: usize = Len::decoder(c)?.into_inner().try_into().unwrap();
        // BTreeMap::

        let five_fives = std::iter::repeat(5).take(5);
        let v = Vec::from_iter(five_fives);
        assert_eq!(v, vec![5, 5, 5, 5, 5]);

        todo!()
        // Ok(())
        // Ok(())
    }
}
