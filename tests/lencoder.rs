use bin_layout::{lencoder::*, DataType};

#[test]
fn test_lencoder_l2() {
    for num in 0..=L2::MAX {
        let mut buf = [0; 2];
        L2(num).encode(&mut buf).unwrap();
        assert_eq!(num, L2::decode(&buf).unwrap().0);
    }
}
#[test]
fn test_lencoder_l3() {
    for num in 0..(L3::MAX / 3) {
        let mut buf = [0; 3];
        L3(num).encode(&mut buf).unwrap();
        assert_eq!(num, L3::decode(&buf).unwrap().0);
    }
}

#[allow(unused)]
macro_rules! dbg_lencoder {
    [$ty:tt, $num:expr] => {
        let mut buf = [0; 4];
        $ty($num).encode(&mut buf).unwrap();
        let mut reader = buf.as_ref().into();
        let decoded: $ty = DataType::deserialize(&mut reader).unwrap();
        println!("\nNumber: {:?} ({:#X})", $num, $num);
        println!("Packed: {:b}\n", $num);
        println!("Encoded Bits:");
        (0..reader.offset).for_each(|i| println!("{:?}: {:8b}", i, buf[i]));
        println!("\n{:?}", decoded);
        println!("Decoded Bits: {:b}\n", decoded.0);
        println!("IsEquel: {}\n", if decoded.0 == $num { '✅' } else { '❌' });
    };
}
