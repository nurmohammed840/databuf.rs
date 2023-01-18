use ::databuf::{Decoder, Encoder};

#[derive(Encoder, Decoder)]
struct Foo;

#[derive(Encoder, Decoder)]
struct Bar<'a, T> {
    field2: &'a str,
    field1: T,
    field0: Foo,
}