#![allow(warnings)]
use databuf::{Decode, Encode};

#[repr(u8)]
#[derive(Encode, Decode)]
enum Enum1 {
    A,
    B,
    E = 2,
    C,
}

// #[derive(Encode, Decode)]
// enum Enum2 {
//     A,
//     B,
//     C,
// }

// #[test]
// fn test_name() {}

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum Number {
    Integer(i64) = 1,
    Float(f64),
    FloatW(f64),
    Complex { real: f64, imaginary: f64 } = 5,
}
