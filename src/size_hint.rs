use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::*;
pub trait SizeHint {
    /// Calculate total estimated size of the data structure in bytes.
    #[inline]
    fn size_hint(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

macro_rules! size_hint {
    [Set] => {
        #[inline] fn size_hint(&self) -> usize {
            len::Len::SIZE + self.iter().map(T::size_hint).sum::<usize>()
        }
    };
    [Map] => {
        #[inline] fn size_hint(&self) -> usize {
            len::Len::SIZE + self .iter()
                .map(|(k, v)| k.size_hint() + v.size_hint())
                .sum::<usize>()
        }
    };
    [$($ty:ty)*] => { $(impl SizeHint for $ty {})* };
    [$($ty:ty: $size:literal)*] => { $(impl SizeHint for $ty { fn size_hint(&self) -> usize { $size } })* };
}

size_hint! {
    len::L2: 2
    len::L3: 3
}
size_hint! {
    bool char
    f32 f64
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
}

impl<T: SizeHint, const N: usize> SizeHint for [T; N] {
    fn size_hint(&self) -> usize {
        self.iter().map(T::size_hint).sum()
    }
}

impl<T: SizeHint> SizeHint for Option<T> {
    fn size_hint(&self) -> usize {
        match self {
            Some(v) => 1 + v.size_hint(),
            None => 1,
        }
    }
}

impl<T: SizeHint, E: SizeHint> SizeHint for std::result::Result<T, E> {
    fn size_hint(&self) -> usize {
        1 + match self {
            Ok(v) => v.size_hint(),
            Err(e) => e.size_hint(),
        }
    }
}

impl<const N: usize> SizeHint for &[u8; N] {
    fn size_hint(&self) -> usize {
        N
    }
}

impl<T: SizeHint> SizeHint for Box<T> {
    fn size_hint(&self) -> usize {
        T::size_hint(self)
    }
}

impl<T> SizeHint for std::marker::PhantomData<T> {
    fn size_hint(&self) -> usize {
        0
    }
}

impl<T: SizeHint> SizeHint for Vec<T> {
    size_hint! {Set}
}

impl<T: SizeHint, S> SizeHint for HashSet<T, S> {
    size_hint! {Set}
}

impl<T: SizeHint> SizeHint for BTreeSet<T> {
    size_hint! {Set}
}

impl<K: SizeHint, V: SizeHint, S> SizeHint for HashMap<K, V, S> {
    size_hint! {Map}
}

impl<K: SizeHint, V: SizeHint> SizeHint for BTreeMap<K, V> {
    size_hint! {Map}
}
