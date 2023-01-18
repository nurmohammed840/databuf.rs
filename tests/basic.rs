use databuf::{Decoder, Encoder};

#[derive(Encoder, Decoder)]
struct Foo;

#[derive(Encoder, Decoder)]
struct Bar<'a> {
    field2: &'a str,
    field1: Foo,
}
