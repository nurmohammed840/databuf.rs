use crate::*;
use lencoder::Lencoder;

macro_rules! impls {
    [$($tys:tt),* : $deserialize:item] => {
        impl<'de> DataType<'de> for $($tys)* {
            #[inline]
            fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.as_ref();
                Lencoder::SIZE + bytes.len()
            }
            #[inline]
            fn serialize(self, view: &mut Cursor<impl Bytes>) {
                Lencoder(self.len().try_into().unwrap()).serialize(view);
                view.write_slice(self);
            }
            #[inline] $deserialize
        }
    };
}

impls!(&, 'de, [u8]:
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let len = Lencoder::deserialize(view)?.0;
        view.read_slice(len as usize)
    }
);

impls!(&, 'de, str: 
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let bytes: &'de [u8] = DataType::deserialize(view)?;
        core::str::from_utf8(bytes).map_err(|_| InvalidUtf8)
    }
);

impls!(String:
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let bytes: &[u8] = DataType::deserialize(view)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| InvalidData)
    }
);

impl<'de, T: DataType<'de>> DataType<'de> for Vec<T> {
    #[inline]
    fn size_hint(&self) -> usize {
        Lencoder::SIZE + self.iter().map(T::size_hint).sum::<usize>()
    }

    #[inline]
    fn serialize(self, view: &mut Cursor<impl Bytes>) {
        Lencoder(self.len().try_into().unwrap()).serialize(view);
        for item in self {
            item.serialize(view);
        }
    }
    
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        let len = Lencoder::deserialize(view)?.0;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::deserialize(view)?);
        }
        Ok(vec)
    }
}
