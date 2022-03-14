use crate::*;

impl DataType for String {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write(len).unwrap();
        view.write_slice(self).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); InsufficientBytes) as usize;
        let bytes = map!(@opt view.read_slice(len); InsufficientBytes).into();
        Ok(map!(@err String::from_utf8(bytes); InvalidData))
    }
}

impl<D> DataType for Vec<D>
where
    D: DataType,
{
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write::<u32>(len).unwrap();
        for item in self {
            item.serialize(view);
        }
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); InsufficientBytes);
        (0..len).map(|_| D::deserialize(view)).collect()
    }
}
