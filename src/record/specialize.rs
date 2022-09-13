use crate::*;

impl Encoder for [u8] {
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        encode_len!(self, c);
        c.write_all(self)
    }
}

impl Encoder for Vec<u8> {
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        encode_len!(self, c);
        c.write_all(self)
    }
}

impl Encoder for std::collections::VecDeque<u8> {
    fn encoder(&self, c: &mut impl Write) -> io::Result<()> {
        encode_len!(self, c);
        let (left, right) = self.as_slices();
        c.write_all(left)?;
        c.write_all(right)
    }
}

