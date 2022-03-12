#![allow(warnings)]

use bin_layout::{utils::Record, DataType, DataView, ErrorKind};
use std::convert::TryInto;

use Subject::*;
#[derive(Debug)]
enum Subject {
    Math,
    Physics,
    Chemistry,
    Other(u16, String),
}

impl DataType for Subject {
    fn serialize(self, view: &mut DataView<impl AsMut<[u8]>>) {
        let code = match self {
            Math => 302,
            Physics => 317,
            Chemistry => 345,
            Other(id, name) => {
                let len: u8 = name.len().try_into().unwrap();
                view.write(id);
                view.write(len);
                return view.write_slice(name);
            }
        };
        view.write::<u16>(code);
    }
    fn deserialize(view: &mut DataView<impl AsRef<[u8]>>) -> Result<Self, ErrorKind> {
        let name = match u16::deserialize(view)? {
            302 => Math,
            317 => Physics,
            345 => Chemistry,
            id => {
                let record: Record<u8, _> = Record::deserialize(view)?;
                return Ok(Other(id, record.data));
            }
        };
        Ok(name)
    }
}

#[derive(Debug, DataType)]
struct Student {
    age: u8,
    name: String,
    gender: bool,
    roll: u16,
}

#[derive(Debug, DataType)]
struct Class {
    name: String,
    subjects: [Subject; 4],
    students: Record<u8, Vec<Student>>,
}

#[test]
fn basic() {
    let class = Class {
        name: "Mango".into(),
        subjects: [Physics, Chemistry, Other(321, "Engish II".into()), Math],
        students: vec![
            Student {
                age: 21,
                name: "John".into(),
                gender: true,
                roll: 73,
            },
            Student {
                age: 20,
                name: "Jona".into(),
                gender: false,
                roll: 36,
            },
        ]
        .into(),
    };

    let before = format!("{:?}", class);

    let mut view = [0; 64].into();
    class.serialize(&mut view);
    view.offset = 0;

    let class: Class = DataType::deserialize(&mut view).unwrap();
    let after = format!("{:?}", class);

    assert_eq!(before, after);
}
