use crate::*;

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de> DataType<'de> for $($tys)* {
            #[inline]
            fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
                #[cfg(not(feature = "L3"))]
                lencoder::L2(self.len() as u16).serialize(view)?;
                #[cfg(feature = "L3")]
                lencoder::L3(self.len() as u32).serialize(view)?;

                view.write_slice(self)
            }
            #[inline]
            $deserialize
        }
    };
}
impls!(&, 'de, [u8] => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    #[cfg(not(feature = "L3"))]
    let len = lencoder::L2::deserialize(view)?.0;
    #[cfg(feature = "L3")]
    let len = lencoder::L3::deserialize(view)?.0;

    view.read_slice(len as usize)
});

impls!(&, 'de, str => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: &'de [u8] = DataType::deserialize(view)?;
    core::str::from_utf8(bytes).map_err(|_| InvalidUtf8)
});

impls!(String => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let bytes: &[u8] = DataType::deserialize(view)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| InvalidData)
});

impl<'de, T: DataType<'de>> DataType<'de> for Vec<T> {
    #[inline]
    fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
        #[cfg(not(feature = "L3"))]
        lencoder::L2(self.len() as u16).serialize(view)?;
        #[cfg(feature = "L3")]
        lencoder::L3(self.len() as u32).serialize(view)?;

        for item in self {
            item.serialize(view)?;
        }
        Ok(())
    }
    #[inline]
    fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
        #[cfg(not(feature = "L3"))]
        let len = lencoder::L2::deserialize(view)?.0;
        #[cfg(feature = "L3")]
        let len = lencoder::L3::deserialize(view)?.0;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::deserialize(view)?);
        }
        Ok(vec)
    }
}
