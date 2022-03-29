[Doc](https://docs.rs/bin-layout/)

Very fast! And flexible, This library used to serialize and deserialize data in binary format.

Inspaired by [bincode](https://github.com/bincode-org/bincode), But much more flexible.

### [Endianness](https://en.wikipedia.org/wiki/Endianness)

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. For example:

```toml
[dependencies]
bin-layout = { version = "3", features = ["BE"] }
```

### Example

```rust
use bin_layout::DataType;

#[derive(DataType)]
struct Car<'a> {
    name: &'a str,  // Zero-Copy deserialization
    year: u16,
    is_new: bool,
}

#[derive(DataType)]
struct Company<'a> { name: String, cars: Vec<Car<'a>> }

let old = Company {
    name: "Tesla".into(),
    cars: vec![
        Car { name: "Model S", year: 2018, is_new: true },
        Car { name: "Model X", year: 2019, is_new: false },
    ],
};

let bytes = old.encode();
let new = Company::decode(&bytes).unwrap();
```

### Data Type

The only trait you need to implement is [DataType](https://docs.rs/bin-layout/latest/bin_layout/trait.DataType.html).

All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this trait.

`Vec`, `String`, `&[T]`, `&str` etc.. are encoded with their length value first, Following by each entry.

#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `L2` and `L3`, both are encoded in little endian.

By default, `L2` (u15) is used to encode length (integer) for record. But you override it by setting `L3` (u22) in features flag.
 
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

#### Fixed-Length Integer Encoding

[Record](https://docs.rs/bin-layout/latest/bin_layout/struct.Record.html) can be used to represent fixed-size integer to represent the length of a record.

It accepts fixed-length unsigned interger type of `N` (`u8`, `u32`, `usize`, etc..) and a generic type of `T` (`Vec<T>`, `String` etc..)