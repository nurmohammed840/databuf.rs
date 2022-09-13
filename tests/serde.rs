use bin_layout::*;

#[derive(Encoder, Decoder, Clone, Debug, PartialEq)]
struct Foo<'a> {
    a: u64,
    b: [u16; 3],
    c: Record<u8, &'a str>,
}

#[test]
fn test_foo() {
    let foo = Foo {
        a: 42,
        b: [1, 2, 3],
        c: "HelloWorld".into(),
    };
    let arr = foo.encode();
    assert_eq!(arr.len(), 22);
    assert_eq!(Foo::decode(&arr).unwrap(), foo);
}

#[test]
fn test_name() {
    let string = String::from("HelloWorld").repeat(26);
    println!("{:?}", string.as_bytes().len());
    let record: Record<u8, _> = Record::new(string);
    let mut c = vec![];
    let result = record.encoder(&mut c);
    println!("{:?}", result);
    println!("{:?}", c.len());
}
