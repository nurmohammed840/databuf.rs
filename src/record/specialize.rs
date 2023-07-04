use crate::*;

impl Encode for [u8] {
    fn encode(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        encode_len!(self, c);
        c.write_all(self)
    }
}

impl Encode for Vec<u8> {
    fn encode(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        encode_len!(self, c);
        c.write_all(self)
    }
}

impl Encode for std::collections::VecDeque<u8> {
    fn encode(&self, c: &mut (impl Write + ?Sized)) -> io::Result<()> {
        encode_len!(self, c);
        let (left, right) = self.as_slices();
        c.write_all(left)?;
        c.write_all(right)
    }
}
