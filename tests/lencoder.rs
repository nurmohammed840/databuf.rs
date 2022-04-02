use bin_layout::{lencoder::*, Decoder, Encoder};

#[test]
fn test_lencoder() -> Result<(), ()> {
    for num in 0..=L2::MAX {
        assert_eq!(num, L2::decode(&L2(num).encode())?.0);
    }
    for num in 0..=(L3::MAX / 3) {
        assert_eq!(num, L3::decode(&L3(num).encode())?.0);
    }
    Ok(())
}

#[allow(unused)]
macro_rules! dbg_lencoder {
    [$ty:tt, $num:expr] => {
        let mut buf = [0; 4];
        $ty($num).encode(&mut buf).unwrap();
        let mut reader = buf.as_ref().into();
        let decoded = (Decoder::decoder(&mut reader) as Result<$ty, ()>).unwrap();
        println!("\nNumber: {:?} ({:#X})", $num, $num);
        println!("Packed: {:b}\n", $num);
        println!("Encoded Bits:");
        (0..reader.offset).for_each(|i| println!("{:?}: {:8b}", i, buf[i]));
        println!("\n{:?}", decoded);
        println!("Decoded Bits: {:b}\n", decoded.0);
        println!("IsEquel: {}\n", if decoded.0 == $num { '✅' } else { '❌' });
    };
}
