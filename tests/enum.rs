#![allow(warnings)]
use databuf::{Decode, Encode};

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

#[repr(usize)]
#[derive(Encode, Decode)]
pub enum Number {
    Integer(i64) = 1,
    Float(f64),
    Complex { real: f64, imaginary: f64 } = 3,
}
