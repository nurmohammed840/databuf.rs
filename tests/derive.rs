use std::io::Write;

use databuf::{config::num::LEB128, *};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Object<'a, T, Byte, const N: usize> {
    buf: [Byte; N],
    unit: Data<'a, T>,
    r#ref: Data<'a, T>,
    data: Data<'a, T>,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Data<'a, T> {
    Unit,
    Ref { data: &'a [u8] },
    Data(T, T),
}

#[test]
fn test_derive() {
    let data = "Hello, World!".as_bytes();
    let obj = Object {
        buf: [1_u8; 42],
        unit: Data::Unit,
        r#ref: Data::Ref { data },
        data: Data::Data(42_u16, 24),
    };
    let mut bytes = vec![];
    {
        let buf: &mut dyn Write = &mut bytes;
        obj.encode::<LEB128>(buf).unwrap();
    }
    let new_obj = Object::from_bytes::<LEB128>(&bytes).unwrap();
    assert_eq!(obj, new_obj);
}
