This library 


### Endian

By default, the library uses little endian.
If you want to use big endian, you can set `BE` features flag. And for native endian use `NE`. for example:

```toml
[dependencies]
bin-layout = { version = "0.1", features = ["BE"] }
```

### Data Types

#### Primitive Types

All supported primitive types are: `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`, `bool`