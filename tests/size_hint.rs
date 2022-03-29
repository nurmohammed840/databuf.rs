#![allow(non_snake_case)]
#![allow(warnings)]

use bin_layout::DataType;

#[derive(DataType)]
struct Byte(u8);

#[derive(DataType)]
struct Buffer<const N: usize>([Byte; N]);

#[derive(DataType)]
struct Unsigned {
    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    U128: u128,
}

#[derive(DataType)]
struct Signed {
    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    I128: i128,
}

#[derive(DataType)]
struct Float {
    short: f32,
    long: f64,
}

#[derive(DataType)]
struct Num {
    integer: Int,
    float: Float,
}

#[derive(DataType)]
struct Int {
    unsigned: Unsigned,
    signed: Signed,
}

#[derive(DataType)]
struct Bool(bool);

#[derive(DataType)]
struct Char(char);

#[derive(DataType)]
struct Primitive {
    number: Num,
    boolen: Bool,
    charecter: Char,
}

#[test]
fn test() {
    assert_eq!(Bool::SIZE, 1);
    assert_eq!(Char::SIZE, 4);

    assert_eq!(Signed::SIZE, 31); // 1 + 2 + 4 + 8 + 16
    assert_eq!(Unsigned::SIZE, 31); // 1 + 2 + 4 + 8 + 16

    assert_eq!(Int::SIZE, 62); // Signed + Unsigned
    assert_eq!(Float::SIZE, 12); // 4 + 8

    assert_eq!(Num::SIZE, 74); // Int + Float

    assert_eq!(Primitive::SIZE, 79); // Num + Bool + Char
    assert_eq!(Primitive::IS_DYNAMIC, false);

    //------------------------------------------
    
    assert_eq!(Byte::SIZE, 1);
    assert_eq!(Buffer::<7>::SIZE, 7);
    assert_eq!(<(Byte, Buffer::<0>)>::SIZE, 1);

    assert_eq!(<&[u8; 42]>::SIZE, 42);
    assert_eq!(<[[u8; 10]; 42]>::SIZE, 420);
}
