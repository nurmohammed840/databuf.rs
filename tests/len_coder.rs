use bin_layout::{
    len_coder::{U15, U22},
    DataType,
};

#[test]
fn test_lencoder_u15() {
    for num in 0..=U15::MAX {
        let mut buf = [0; 2];
        U15(num).encode(&mut buf);
        assert_eq!(num, U15::decode(&buf).unwrap().0);
    }
}

#[test]
fn test_lencoder_u22() {
    for num in 0..=U22::MAX {
        let mut buf = [0; 3];
        U22(num).encode(&mut buf);
        assert_eq!(num, U22::decode(&buf).unwrap().0);
    }
}


#[allow(unused)]
macro_rules! debug_len_coder {
    [$ty:tt, $num:expr] => {
        let mut buf = [0; 3];
        $ty($num).encode(&mut buf);
        let mut reader = buf.as_ref().into();
        let decoded: $ty = DataType::deserialize(&mut reader).unwrap();
        println!("\nNumber: {:?}", $num);
        println!("Packed: {:b}\n", $num);
        println!("Encoded Bits:");
        (0..reader.offset).for_each(|i| println!("{:?}: {:8b}", i, buf[i]));
        println!("\n{:?}", decoded);
        println!("Decoded Bits: {:b}\n", decoded.0);
        println!("IsEquel: {}\n", if decoded.0 == $num { '✅' } else { '❌' });
    };
}

#[test]
#[allow(unused)]
fn debug_playground() {
    debug_len_coder!(U15, 32767);
}
