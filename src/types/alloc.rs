use crate::*;

impl DataType for String {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write(len).unwrap(); 
        view.write_slice(self).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); NotEnoughData) as usize;
        let bytes = map!(@opt view.read_slice(len); NotEnoughData).into();
        Ok(map!(@err String::from_utf8(bytes); InvalidValue))
    }
}

impl<D: DataType> DataType for Vec<D> {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write::<u32>(len).unwrap(); 
        for item in self {
            item.serialize(view);
        }
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        let len = map!(@opt view.read::<u32>(); NotEnoughData);
        (0..len).map(|_| D::deserialize(view)).collect()
    }
}