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

- Zero-copy deserialization: mean that no data is copied. `Vec`, `String`, `&[T]`, `&str` etc.. are encoded with their length value first, Following by each entry.
    
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
/// Use big endian byte order + Encode `msg` length with `databuf::var_int::LEU15` 
const CONFIG: u8 = num::BE | len::LEU15;

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

#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `LEU15`, `LEU22`, `LEU29`, Encoded in little endian.
By default, `LEU29` is used to encode length.
 
Encoding algorithm is very straightforward,
The most significant bits of the first byte determine the byte length to encode the number in little endian.

#### LEU15

|  MSB  | Length | Usable Bits | Range    |
| :---: | :----: | :---------: | :------- |
|   0   |   1    |      7      | 0..128   |
|   1   |   2    |     15      | 0..32768 |

#### LEU22

|  MSB  | Length | Usable Bits | Range      |
| :---: | :----: | :---------: | :--------- |
|   0   |   1    |      7      | 0..128     |
|  10   |   2    |     14      | 0..16384   |
|  11   |   3    |     22      | 0..4194304 |

#### LEU29

|  MSB   | Length | Usable Bits | Range        |
| :---:  | :----: | :---------: | :----------- |
|  0     |   1    |      7      | 0..128       |
|  10    |   2    |     14      | 0..16384     |
|  110   |   3    |     21      | 0..2097152   |
|  111   |   4    |     29      | 0..536870912 |

 
For example, Binary representation of `0x_C0DE` is `0x_11_00000011_011110`
 
`LEU22(0x_C0DE)` is encoded in 3 bytes:
 
```yml
1st byte: 11_011110      # MSB is 11, so read next 2 bytes
2nd byte:        11
3rd byte:        11
```

Another example, `LEU22(107)` is encoded in just 1 byte:

```yml
1st byte: 0_1101011      # MSB is 0, So we don't have to read extra bytes.
```