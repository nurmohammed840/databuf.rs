#![allow(non_snake_case)]
#![allow(warnings)]

use bin_layout::{Encoder, Decoder};

#[derive(Encoder, Decoder)]
struct Byte(u8);

#[derive(Encoder, Decoder)]
struct Buffer<const N: usize>([Byte; N]);


#[derive(Encoder, Decoder)]
struct Unsigned {
    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    U128: u128,
}

#[derive(Encoder, Decoder)]
struct Signed {
    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    I128: i128,
}

#[derive(Encoder, Decoder)]
struct Float {
    short: f32,
    long: f64,
}

#[derive(Encoder, Decoder)]
struct Num {
    integer: Int,
    float: Float,
}

#[derive(Encoder, Decoder)]
struct Int {
    unsigned: Unsigned,
    signed: Signed,
}

#[derive(Encoder, Decoder)]
struct Bool(bool);

#[derive(Encoder, Decoder)]
struct Char(char);

#[derive(Encoder, Decoder)]
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

    //------------------------------------------

    assert_eq!(Byte::SIZE, 1);
    assert_eq!(Buffer::<7>::SIZE, 7);
    assert_eq!(<(Byte, Buffer::<0>)>::SIZE, 1);

    assert_eq!(<&[u8; 42]>::SIZE, 42);
    assert_eq!(<[[u8; 10]; 42]>::SIZE, 420);
}
