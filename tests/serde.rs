use bin_layout::*;
use stack_array::ArrayBuf;

#[derive(Encoder, Decoder, PartialEq, Clone, Debug)]
struct ABC<'a> {
    a: u64,
    b: [u8; 3],
    c: Record<u8, &'a str>,
}

#[test]
fn serde_test() {
    let abc = ABC {
        a: 42,                  // 8 bytes
        b: [1, 2, 3],           // 3 bytes
        c: "HelloWorld".into(), // 1 + 10 bytes
    };
    assert_eq!(abc.size_hint(), 22);

    let mut arr: ArrayBuf<u8, 30> = ArrayBuf::new();
    abc.clone().encoder(&mut arr);

    assert_eq!(arr.len(), 22);

    let new_abc: Result<_, ErrorKind> = ABC::decode(&arr);
    assert_eq!(new_abc.unwrap(), abc);
}
