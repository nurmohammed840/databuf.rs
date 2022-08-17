// use bin_layout::*;
// use stack_array::ArrayBuf;

// #[derive(Encoder, Decoder, PartialEq, Clone, Debug)]
// struct Foo<'a> {
//     a: u64,
//     b: [u8; 3],
//     c: Record<u8, &'a str>,
// }

// #[test]
// fn test_foo() {
//     let abc = Foo {
//         a: 42,                  // 8 bytes
//         b: [1, 2, 3],           // 3 bytes
//         c: "HelloWorld".into(), // 1 + 10 bytes
//     };
//     assert_eq!(abc.size_hint(), 22);

//     let mut arr: ArrayBuf<u8, 30> = ArrayBuf::new();
//     abc.encoder(&mut arr);

//     assert_eq!(arr.len(), 22);
//     assert_eq!(Foo::decode(&arr).unwrap(), abc);
// }

// #[derive(Encoder, Decoder, Debug)]
// struct All {
//     i128: i128,
//     i64: i64,
//     i32: i32,
//     i16: i16,
//     i8: i8,
//     u8: u8,
//     u16: u16,
//     u32: u32,
//     u64: u64,
//     u128: u128,
//     bool_true: bool,
//     bool_false: bool,
//     char: char
// }

// macro_rules! to_bytes {
//     [$num: expr] => {
//         if cfg!(not(any(feature = "BE", feature = "NE"))) {
//             $num.to_le_bytes()
//         } else if cfg!(feature = "BE") {
//             $num.to_be_bytes()
//         } else {
//             $num.to_ne_bytes()
//         }
//     };
// }

// fn test() -> Option<()> {
//     let all = All {
//         i128: -5,
//         i64: -4,
//         i32: -3,
//         i16: -2,
//         i8: -1,
//         u8: 0,
//         u16: 1,
//         u32: 2,
//         u64: 3,
//         u128: 4,
//         bool_true: true,
//         bool_false: false,
//         char: 'a',
//     };

//     let bytes = all.encode();
//     let mut c = Cursor::new(&bytes[..]);

//     assert_eq!(c.read_slice(16)?, to_bytes!(-5_i128));
//     assert_eq!(c.read_slice(8)?, to_bytes!(-4_i64));
//     assert_eq!(c.read_slice(4)?, to_bytes!(-3_i32));
//     assert_eq!(c.read_slice(2)?, to_bytes!(-2_i16));
//     assert_eq!(c.read_slice(1)?, to_bytes!(-1_i8));

//     //-------------------------------------------------------

//     assert_eq!(c.read_slice(1)?, to_bytes!(0_u8));
//     assert_eq!(c.read_slice(2)?, to_bytes!(1_u16));
//     assert_eq!(c.read_slice(4)?, to_bytes!(2_u32));
//     assert_eq!(c.read_slice(8)?, to_bytes!(3_u64));
//     assert_eq!(c.read_slice(16)?, to_bytes!(4_u128));

//     // ------------------------------------------------------

//     assert_eq!(c.read_slice(1)?, [1]); // true
//     assert_eq!(c.read_slice(1)?, [0]); // false

//     // ------------------------------------------------------

//     assert_eq!(c.read_slice(4)?, to_bytes!(u32::from('a')));

//     assert!(c.remaining_slice().is_empty());
//     Some(())
// }

// #[test]
// fn test_all() {
//     test().unwrap();
// }
