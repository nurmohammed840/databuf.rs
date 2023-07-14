[Doc](https://docs.rs/databuf/)

This library used to serialize and deserialize structured data in binary format.

### Examples

```toml
[dependencies]
databuf = "0.3"
```

```rust
use databuf::{*, config::num::LE};

#[derive(Encode, Decode)]
struct Car<'a> {
    year: u16,
    is_new: bool,
    name: &'a str,
}

#[derive(Encode, Decode)]
struct Company<'a> { name: String, cars: Vec<Car<'a>> }

let old = Company {
    name: "Tesla".into(),
    cars: vec![
        Car { name: "Model S", year: 2018, is_new: true },
        Car { name: "Model X", year: 2019, is_new: false },
    ],
};
let bytes = old.to_bytes::<LE>();
let new = Company::from_bytes::<LE>(&bytes).unwrap();
```

`Vec`, `String`, `&[T]`, `&str` etc.. are encoded with their length value first, Following by each entry.

By default, length of collections is represented with `BEU30`.
    
```rust
use databuf::{*, config::num::LE};

#[derive(Encode, Decode)]
struct Msg<'a> {
    id: u16,
    data: &'a str,
}
let bytes = [42, 0, 13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
//           ^^^^^  ^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//            Id    Len                         Data

let msg = Msg::from_bytes::<LE>(&bytes).unwrap();
assert_eq!(msg.id, 42);
assert_eq!(msg.data, "Hello, World!"); // Here, data is referenced.
```

- Example: Encoding data into a buffer of specified size.

```rust
use databuf::{*, config::{num, len}};
/// Use big endian byte order + Encode `msg` length with `databuf::var_int::BEU15` 
const CONFIG: u16 = num::BE | len::BEU15;

#[derive(Encode, Decode)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Encode, Decode)]
struct Record<T> {
    id: T,
    date: Date,
    msg: String,
}

let record = Record { id: 42_u32, date: Date { year: 2018, month: 3, day: 7 }, msg: "Hello!".into() };

let mut buf = [0; 20];
let remaining = &mut buf.as_mut_slice();
record.encode::<CONFIG>(remaining).unwrap();

let amt = 20 - remaining.len();
assert_eq!(amt, 15); // 15 bytes written to `buf`
```
