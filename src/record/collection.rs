use crate::*;
use std::{
    collections::*,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};
macro_rules! impl_v2 {
    [Encoder for $name: ty where $($ty: tt)*] => {
        impl<$($ty)*> Encoder for $name { impl_v2! {@EncoderBody} }
        impl<Len: LenType, $($ty)*> Encoder for Record<Len, $name>
        where
            Len::Error: Into<DynErr>,
        {
            impl_v2! {@EncoderBody}
        }
    };
    [Decoder for $name: ty where $($ty: tt)*] => {
        impl<'de, $($ty)*> Decoder<'de> for $name { impl_v2! {@DecoderBody} }
        impl<'de, Len: LenType,  $($ty)*> Decoder<'de> for Record<Len, $name>
        where
            usize: TryFrom<Len>,
            <usize as TryFrom<Len>>::Error: Into<DynErr>,
        {
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
        #[inline] fn decoder(cursor: &mut &'de [u8]) -> Result<Self> {
            let len = Len::decoder(cursor)?.try_into().map_err(invalid_input)?;
            let mut error = None;
            let out = Self::from_iter(Iter { len, err: &mut error, cursor, _marker: PhantomData });
            match error {
                Some(err) => Err(err),
                None => Ok(out),
            }
        }
    }
}

impl<T: Encoder> Encoder for [T] {
    impl_v2! {@EncoderBody}
}

impl<Len: LenType, T: Encoder> Encoder for Record<Len, &[T]>
where
    Len::Error: Into<DynErr>,
{
    impl_v2! {@EncoderBody}
}

//--------------------------------------------------------------------

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

// --------------------------------------------------------------------------------

struct Iter<'err, 'c, 'de, T> {
    len: usize,
    err: &'err mut Option<std::io::Error>,
    cursor: &'c mut &'de [u8],
    _marker: PhantomData<T>,
}

impl<'err, 'c, 'de, T: Decoder<'de>> Iterator for Iter<'err, 'c, 'de, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        match T::decoder(self.cursor) {
            Ok(val) => Some(val),
            Err(err) => {
                self.len = 0;
                *self.err = Some(err);
                None
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
