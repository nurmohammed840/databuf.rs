[Doc](https://docs.rs/databuf/)

This library used to serialize and deserialize structured data in binary format.

### Examples

```toml
[dependencies]
databuf = "0.2"
```

```rust
use databuf::*;

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
let bytes = old.to_bytes();
let new = Company::from_bytes(&bytes);
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

- In this example, The following structs, don't have any dynamic length data. So we can have a fixed size buffer at compile time.

```rust
use databuf::*;

#[derive(Encode, Decode)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Encode, Decode)]
struct Record {
    id: u32,
    date: Date,
    value: [u8; 512],
}

let record = Record { id: 42, date: Date { year: 2018, month: 3, day: 7 }, value: [1; 512] };
let mut writer = [0; 520];
record.encode(&mut writer.as_mut_slice());
```

- It's very easy to implement `Encode` or `Decode` trait. For example:

```rust
use std::io;
use databuf::*;

#[derive(Encode, Decode)]
struct Bar(u16);
struct Foo { x: u8, y: Bar }

impl Encode for Foo {
    fn encode(&self, c: &mut impl io::Write) -> io::Result<()> {
        self.x.encode(c)?;
        self.y.encode(c)
    }
}
impl Decode<'_> for Foo {
    fn decode(c: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            x: u8::decode::<CONFIG>(c)?,
            y: Bar::decode::<CONFIG>(c)?,
        })
    }
}
```

#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `LEU15` and `LEU22`, both are encoded in little endian.

By default, `LEU22` (u22) is used to encode length (integer) for record. But you override it by setting `LEU15` (u15) in features flag.
 
Encoding algorithm is very straightforward, reserving one or two most significant bits of the first byte to encode rest of the length.

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

#### Fixed-Length Collections

[Record](https://docs.rs/databuf/latest/databuf/struct.Record.html) can be used to 
encode collections where the size of the length is known. 

For example, `Record<u8, String>` here the maximum allowed payload length is 255 (`u8::MAX`)
