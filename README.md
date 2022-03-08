Very fast (Zero Cost)! yet flexible, this library used to serialize and deserialize data in binary format.

### Endian

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. for example:

```toml
[dependencies]
bin-layout = { version = "0.1", features = ["BE"] }
```

### Data Types

The library is very flexible and easy to use. The only trait you need to implement is `DataType`.

All [primitive types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html) (Both Scalar and Compound Types) implement this `DataType` trait.

And For collection types, Only `Vec` and `String` are supported. 