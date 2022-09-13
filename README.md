[Doc](https://docs.rs/bin-layout/)

Very fast! And flexible, This library used to serialize and deserialize data in binary format.

### [Endianness](https://en.wikipedia.org/wiki/Endianness)

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. For example:

```toml
[dependencies]
bin-layout = { version = "7", features = ["BE"] }
```

### Examples

```rust
use bin_layout::*;

#[derive(Encoder, Decoder)]
struct Car<'a> {
    year: u16,
    is_new: bool,
    name: &'a str,
}

#[derive(Encoder, Decoder)]
struct Company<'a> { name: String, cars: Vec<Car<'a>> }

let old = Company {
    name: "Tesla".into(),
    cars: vec![
        Car { name: "Model S", year: 2018, is_new: true },
        Car { name: "Model X", year: 2019, is_new: false },
    ],
};
let bytes = old.encode();
let new = Company::decode(&bytes);
```

- Zero-copy deserialization: mean that no data is copied. Dynamic length data (`Vec`, `String`, `&[T]`, `&str` etc..) are encoded with their length value first, Following by each entry.
    
```rust
use bin_layout::*;

#[derive(Encoder, Decoder)]
struct Msg<'a> {
    id: u8,
    data: &'a str,
}
let bytes = [42, 13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
//           ^^  ^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//           Id  Len                         Data

let msg = Msg::decode(&bytes).unwrap();
assert_eq!(msg.id, 42);
assert_eq!(msg.data, "Hello, World!"); // Here, data is referenced.
```

- In this example, The following structs, don't have any dynamic length data. So we can have a fixed size buffer at compile time.

```rust
use bin_layout::*;

#[derive(Encoder, Decoder)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Encoder, Decoder)]
struct Record {
    id: u32,
    date: Date,
    value: [u8; 512],
}

let record = Record { id: 42, date: Date { year: 2018, month: 3, day: 7 }, value: [1; 512] };
let mut writer = [0; 520];
record.encoder(&mut writer.as_mut_slice());
```

- It's very easy to implement `Encoder` or `Decoder` trait. For example:

```rust
use std::io;
use bin_layout::*;

type DynErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Encoder, Decoder)]
struct Bar(u16);
struct Foo { x: u8, y: Bar }

impl Encoder for Foo {
    fn encoder(&self, c: &mut impl io::Write) -> io::Result<()> {
        self.x.encoder(c)?;
        self.y.encoder(c)
    }
}
impl Decoder<'_> for Foo {
    fn decoder(c: &mut &[u8]) -> Result<Self, DynErr> {
        Ok(Self {
            x: u8::decoder(c)?,
            y: Bar::decoder(c)?,
        })
    }
}
```

#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `L2` and `L3`, both are encoded in little endian.

By default, `L3` (u22) is used to encode length (integer) for record. But you override it by setting `L2` (u15) in features flag.
 
Encoding algorithm is very straightforward, reserving one or two most significant bits of the first byte to encode rest of the length.

#### L2

|  MSB  | Length | Usable Bits | Range    |
| :---: | :----: | :---------: | :------- |
|   0   |   1    |      7      | 0..127   |
|   1   |   2    |     15      | 0..32767 |

#### L3

|  MSB  | Length | Usable Bits | Range      |
| :---: | :----: | :---------: | :--------- |
|   0   |   1    |      7      | 0..127     |
|  10   |   2    |     14      | 0..16383   |
|  11   |   3    |     22      | 0..4194303 |

 
For example, Binary representation of `0x_C0DE` is `0x_11_00000011_011110`
 
`L3(0x_C0DE)` is encoded in 3 bytes:
 
```yml
1st byte: 11_011110      # MSB is 11, so read next 2 bytes
2nd byte:        11
3rd byte:        11
```

Another example, `L3(107)` is encoded in just 1 byte:

```yml
1st byte: 0_1101011      # MSB is 0, So we don't have to read extra bytes.
```

#### Fixed-Length Collections

[Record](https://docs.rs/bin-layout/latest/bin_layout/struct.Record.html) can be used to 
encode collections where the size of the length is known. 

For example, `Record<u8, String>` here the maximum allowed payload length is 255 (`u8::MAX`)