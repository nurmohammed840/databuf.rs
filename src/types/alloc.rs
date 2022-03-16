use crate::*;

macro_rules! impls {
    [$($tys:tt),* => $deserialize:item] => {
        impl<'de> DataType<'de> for $($tys)* {
            #[inline]
            fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
                let len: u32 = self.len().try_into().unwrap();
                view.write(len).unwrap();
                view.write_slice(self).unwrap();
            }
            #[inline]
            $deserialize
        }
    };
}

impls!(&, 'de, [u8] => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    view.read::<u32>()
        .and_then(|len| view.read_slice(len as usize))
        .ok_or(InsufficientBytes)
});

impls!(&, 'de, str => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    let bytes: &'de [u8] = DataType::deserialize(view)?;
    core::str::from_utf8(bytes).map_err(|_| InvalidUtf8)
});

impls!(String => fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
    let bytes: &[u8] = DataType::deserialize(view)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| InvalidData)
});

impl<'de, T: DataType<'de>> DataType<'de> for Vec<T> {
    #[inline]
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write(len).unwrap();
        for item in self {
            item.serialize(view);
        }
    }
    #[inline]
    fn deserialize(view: &mut DataView<&'de [u8]>) -> Result<Self> {
        view.read::<u32>()
            .ok_or(InsufficientBytes)
            .and_then(|len| (0..len).map(|_| T::deserialize(view)).collect())
    }
}
