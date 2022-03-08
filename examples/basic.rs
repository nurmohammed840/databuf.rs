// use bin_layout::{def, utils::Record, DataType, DataView, Result};
// use std::convert::TryInto;

// use Subject::*;
// #[derive(Debug)]
// enum Subject {
//     Math,
//     English,
//     Physics,
//     Chemistry,
//     Other(u16, String),
// }

// impl DataType for Subject {
//     fn serialize<T: AsMut<[u8]>>(&self, view: &mut DataView<T>) {
//         let code = match self {
//             Math => 302,
//             English => 310,
//             Physics => 317,
//             Chemistry => 345,
//             Other(id, name) => {
//                 view.write(*id);
//                 // let record: Record<u8, String> = name.into();
//                 // same as `return Record::<u8, String>::serialize(name, view);`
//                 // return record.serialize(view);
//                 let len: u8 = name.len().try_into().unwrap();
//                 view.write(len);
//                 return view.write_slice(name);
//             }
//         };
//         view.write::<u16>(code);
//     }
//     fn deserialize<T: AsRef<[u8]>>(view: &mut DataView<T>) -> Result<Self> {
//         let name = match u16::deserialize(view)? {
//             302 => Math,
//             310 => English,
//             317 => Physics,
//             345 => Chemistry,
//             id => {
//                 let vec: Record<u8, _> = Record::deserialize(view)?;
//                 let name = String::from_utf8(vec.data).unwrap();
//                 return Ok(Other(id, name));
//             }
//         };
//         return Ok(name);
//         todo!()
//     }
// }

// def!(Student, {
//     age: u8,
//     name: String,
//     gender: bool,
//     roll: u16,
// });

// def!(Class, {
//     name: String,
//     subjects: [Subject; 4],
//     students: Record<u8, Vec<Student>>
// });

// fn main() {
//     let mut bytes = [0; 512].into();

//     let packet = Class::deserialize(&mut bytes);
//     let data_section = bytes.remaining_slice();

//     println!("{:#?}", packet);
//     println!("{:?}", data_section);
// }

fn main() {
    
}
