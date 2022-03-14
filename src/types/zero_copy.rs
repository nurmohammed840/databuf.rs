// use crate::*;

// impl<'view> DataType<'view> for &'view [u8] {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         let len: u32 = self.len().try_into().unwrap();
//         view.write(len).unwrap();
//         view.write_slice(self).unwrap()
//     }
//     fn deserialize(view: &'view mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         let len = map!(@opt view.read::<u32>(); InsufficientBytes);
//         let bytes = map!(@opt view.read_slice(len as usize); InsufficientBytes);
//         Ok(bytes)
//     }
// }

// impl<'a> DataType<'a> for &'a str {
//     fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
//         let len: u32 = self.len().try_into().unwrap();
//         view.write(len).unwrap();
//         view.write_slice(self).unwrap();
//     }

//     fn deserialize(view: &'a mut DataView<impl AsRef<[u8]>>) -> Result<Self> {
//         let len = map!(@opt view.read::<u32>(); InsufficientBytes);
//         let bytes = map!(@opt view.read_slice(len as usize); InsufficientBytes);
//         Ok(unsafe { map!(@err core::str::from_utf8(bytes); InvalidData) })
//     }
// }
