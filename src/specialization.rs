use crate::*;

impl Encoder for Vec<u8> {
    fn encoder(&self, _: &mut impl Write) -> Result<()> {
        todo!()
    }
}