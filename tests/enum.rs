use databuf::{Decode, Encode};

#[derive(Encode, Decode)]
enum Enum1 {
    A = 1,
    B,
    C = 3,
}

#[derive(Encode, Decode)]
enum Enum2 {
    A,
    B,
    C,
}

#[test]
fn test_name() {}

#[repr(usize)]
#[derive(Encode, Decode)]
pub enum Number {
    Integer(i64) = 1,
    Float(f64) = 2,
    Complex { real: f64, imaginary: f64 } = 3,
}
