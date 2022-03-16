[Doc](https://docs.rs/bin-layout/)

Very fast! And flexible, This library used to serialize and deserialize data in binary format.

Inspaired by [bincode](https://github.com/bincode-org/bincode), But much more flexible.

### [Endianness](https://en.wikipedia.org/wiki/Endianness)

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. For example:

```toml
[dependencies]
bin-layout = { version = "1", features = ["BE"] }
```

### Data Type

The only trait you need to implement is [DataType](https://docs.rs/bin-layout/latest/bin_layout/trait.DataType.html).

All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this trait.

And For collection types, `Vec` and `String` are supported. They are encoded with their length `u32` value first, Following by each entry of the collection.

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

let company = Company {
    name: "Tesla".into(),
    cars: vec![
        Car { name: "Model S", year: 2018, is_new: true },
        Car { name: "Model X", year: 2019, is_new: false },
    ],
};

let mut buf = [0; 64];

company.encode(buf.as_mut());
let company = Company::decode(buf.as_ref()).unwrap();
```