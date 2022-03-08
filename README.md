Very fast! yet flexible, this library used to serialize and deserialize data in binary format.

Inspaired by [bincode](https://github.com/bincode-org/bincode), But much more flexible.

### Endian

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. for example:

```toml
[dependencies]
bin-layout = { version = "0.1", features = ["BE"] }
```

### Data Types

The library is very flexible and easy to use. The only trait you need to implement is `DataType`.

All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) implement this `DataType` trait.

And For collection types, Only `Vec` and `String` are supported. 

### Example

```rust
use bin_layout::{DataType, def};

def!(Car, { name: String, year: u16, is_new: bool, });
def!(Company, { name: String, cars: Vec<Car>, });

let company = Company {
    name: "Tesla".into(),
    cars: vec![
        Car { name: "Model S".into(), year: 2018, is_new: true },
        Car { name: "Model X".into(), year: 2019, is_new: false },
    ],
};

let mut view = [0; 64].into();
company.serialize(&mut view);

view.offset = 0;

let company = Company::deserialize(&mut view).unwrap();
println!("{:#?}", company);
```

##### Todo

- [ ] Support [Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)