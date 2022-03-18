use crate::*;

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de> DataType<'de> for $($tys)* {
            #[inline]
            fn serialize(self, view: &mut Cursor<impl AsMut<[u8]>>) -> Result<()> {
                let len: u32 = self.len().try_into().unwrap();
                len.serialize(view)?;
                view.write_slice(self)
            }
            #[inline]
            $deserialize
        }
    };
}

impls!(&, 'de, [u8] => fn deserialize(view: &mut Cursor<&'de [u8]>) -> Result<Self> {
    let len = u32::deserialize(view)?;
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
    fn serialize(self, view: &mut cursor::Cursor<impl AsMut<[u8]>>) -> Result<()> {
        let len: u32 = self.len().try_into().unwrap();
        len.serialize(view)?;

        for item in self {
            item.serialize(view)?;
        }
        Ok(())
    }
    #[inline]
    fn deserialize(view: &mut cursor::Cursor<&'de [u8]>) -> Result<Self> {
        let len = u32::deserialize(view)?;
        (0..len).map(|_| T::deserialize(view)).collect()
    }
}