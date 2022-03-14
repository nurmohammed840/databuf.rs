use crate::*;
use ErrorKind::*;

impl DataType for String {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let len: u32 = self.len().try_into().unwrap();
        view.write(len).unwrap();
        view.write_slice(self).unwrap();
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
        view.read::<u32>()
            .ok_or(InsufficientBytes)
            .and_then(|len| view.read_slice(len as usize).ok_or(InsufficientBytes))
            .and_then(|bytes| String::from_utf8(bytes.to_vec()).map_err(|_| InvalidData))
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
        view.read::<u32>()
            .ok_or(InsufficientBytes)
            .and_then(|len| (0..len).map(|_| D::deserialize(view)).collect())
    }
}
