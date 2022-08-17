use std::io::{Result, Write};

use bin_layout::*;
use Subject::*;

#[derive(PartialEq, Debug, Clone)]
enum Subject<'a> {
    Math,
    Physics,
    Chemistry,
    Other(u16, &'a str),
}

impl<'a> Encoder for Subject<'a> {
    fn size_hint(&self) -> usize {
        match *self {
            Other(a, r) => (a, r).size_hint(),
            _ => 2,
        }
    }
    fn encoder(&self, w: &mut impl Write) -> Result<()> {
        let code: u16 = match *self {
            Math => 302,
            Physics => 317,
            Chemistry => 345,
            Other(id, name) => return (id, name).encoder(w),
        };
        code.encoder(w)
    }
}
impl<'de> Decoder<'de> for Subject<'de> {
    fn decoder(c: &mut &'de [u8]) -> Result<Self> {
        let name = match u16::decoder(c)? {
            302 => Math,
            317 => Physics,
            345 => Chemistry,
            id => return Ok(Other(id, Decoder::decoder(c)?)),
        };
        Ok(name)
    }
}

#[derive(PartialEq, Encoder, Decoder, Debug, Clone)]
struct Student<'a> {
    age: u8,
    name: &'a str,
    gender: bool,
    roll: u8,
}

#[derive(Encoder, Decoder, Debug, PartialEq, Clone)]
struct Class<'a> {
    name: &'a str,
    subjects: [Subject<'a>; 4],
    students: Vec<Student<'a>>,
}

#[test]
fn basic() -> Result<()> {
    let old_class = Class {
        name: "Mango",
        subjects: [Physics, Chemistry, Other(321, "Engish II".into()), Math],
        students: vec![
            Student {
                age: 21,
                name: "John",
                gender: true,
                roll: 73,
            },
            Student {
                age: 20,
                name: "Jui",
                gender: false,
                roll: 36,
            },
        ]
        .into(),
    };

    // Note: Size hint for `&str` is `Lencoder::SIZE + bytes.len()`
    // "Mango" has 5 chars + Lencoder::SIZE (L2) = 7
    assert!(old_class.size_hint() >= 43);

    let bytes = old_class.clone().encode();
    assert_eq!(bytes.len(), 40);

    let new_class = Class::decode(&bytes)?;
    assert_eq!(old_class, new_class);
    Ok(())
}

#[derive(Encoder, Decoder)]
struct Unit;

#[derive(Encoder, Decoder)]
struct EmpatyTupleUnit();

