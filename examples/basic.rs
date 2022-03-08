use bin_layout::{def, DataType};

def!(IcmpPacket, {
    request_type: u8,
    metadata: [u16; 5]
});

fn main() {
    let mut bytes = [0; 512].into();

    let packet = IcmpPacket::deserialize(&mut bytes);
    let data_section = bytes.remaining_slice();

    println!("{:#?}", packet);
    println!("{:?}", data_section);
}
