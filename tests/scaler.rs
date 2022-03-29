#[test]
fn test_scaler_type_serialization() {
    use bin_layout::DataType;

    [0x_A5C11, 0x_C0DE, 0x_DEC0DE, 0x_ADDED, 0x_AB0DE, 0x_CAFE]
        .iter()
        .for_each(|&word| {
            assert_eq!(word, u32::decode(word.encode().as_ref()).unwrap());
        });

    [
        0x_DEAD_BEEF,
        0x_Faded_Face,
        0x_BAD_F00D,
        0x_C01D_C0FFEE,
        0x_C0CA_C01A,
    ]
    .iter()
    .for_each(|&word| {
        assert_eq!(word, u64::decode(word.encode().as_ref()).unwrap());
    });
}

mod adw {
    use bin_layout::{Bytes, Cursor, DataType, ErrorKind};
    // Some fixed len data structures

    #[derive(DataType)]
    struct Bar(u16);

    struct Foo {
        x: u8,
        y: Bar,
    }
    impl DataType<'_> for Foo {
        fn serialize(self, c: &mut Cursor<impl Bytes>) {
            self.x.serialize(c);
            self.y.serialize(c);
        }
        fn deserialize(c: &mut Cursor<&[u8]>) -> Result<Self, ErrorKind> {
            Ok(Self {
                x: u8::deserialize(c)?,
                y: Bar::deserialize(c)?,
            })
        }
    }
}
