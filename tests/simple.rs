#![allow(warnings)]
use bin_layout::DataType;

// #[derive(DataType)]
struct Foo<'a>(&'a [u8], &'a [u8], u8);

impl<'de: 'a, 'a> DataType<'de> for Foo<'a> {

    fn serialize(self, cursor: &mut bin_layout::Cursor<impl bin_layout::Bytes>) {
        bin_layout::DataType::serialize(self.0, cursor);
        bin_layout::DataType::serialize(self.1, cursor);
    }
    fn deserialize(_cursor: &mut bin_layout::Cursor<&'de [u8]>) -> bin_layout::Result<Self> {
        todo!()
    }
}
