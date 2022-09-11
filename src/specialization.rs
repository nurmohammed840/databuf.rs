use crate::*;


impl Encoder for [u8] {
    fn encoder(&self, _: &mut impl Write) -> Result<()> {
        todo!()
    }
}

#[test]
fn test_name() {
    let s = Box::<[u8]>::decode(&vec![0, 1, 2]);
    println!("{:?}", s);
}
