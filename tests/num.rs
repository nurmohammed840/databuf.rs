use databuf::var_int::{LEU15, LEU22, LEU29};
use databuf::{
    config::num::{LE, LEB128},
    Decode, Encode,
};

#[test]
fn test_leb128() {
    macro_rules! assert_leb128 {
        ($num: expr) => {{
            let bytes = $num.to_bytes::<LEB128>();
            assert_eq!($num, Decode::from_bytes::<LEB128>(&bytes).unwrap());
            bytes
        }};
    }
    macro_rules! test_leb128 {
        [$($rty:tt)*] => ($(
            let mut bytes = assert_leb128!($rty::MAX);
            bytes[0] += 1;
            assert_eq!(assert_leb128!($rty::MIN), bytes);
        )*);
    }
    assert_eq!(assert_leb128!(u16::MIN), vec![0]);

    assert_eq!(assert_leb128!(u16::MAX), vec![255, 255, 3]);
    assert_eq!(assert_leb128!(u32::MAX), vec![255, 255, 255, 255, 15]);
    assert_leb128!(u64::MAX);
    assert_leb128!(u128::MAX);

    assert_eq!(vec![255, 255, 255, 251, 15], assert_leb128!(f32::MIN));
    assert_eq!(vec![255, 255, 255, 251, 7], assert_leb128!(f32::MAX));
    assert_leb128!(f64::MIN);
    assert_leb128!(f64::MAX);

    test_leb128!(i16 i32 i64 i128);
}

macro_rules! assert_varint {
    [$len: expr, $expect: expr] => {
        let bytes = $len.to_bytes::<LE>();
        assert_eq!(bytes, $expect);
        assert_eq!($len, Decode::from_bytes::<LE>(&bytes).unwrap());
    };
}

#[test]
fn test_le_u15() {
    assert_varint!(LEU15(0), [0]);
    assert_varint!(LEU15(127), [127]);

    assert_varint!(LEU15(128), [128, 1]);
    assert_varint!(LEU15(32767), [255, 255]);
}

#[test]
fn test_le_u22() {
    assert_varint!(LEU22(0), [0]);
    assert_varint!(LEU22(127), [127]);

    assert_varint!(LEU22(128), [128, 2]);
    assert_varint!(LEU22(16383), [191, 255]);

    assert_varint!(LEU22(16384), [192, 0, 1]);
    assert_varint!(LEU22(4194303), [255, 255, 255]);
}

#[test]
fn test_le_u29() {
    assert_varint!(LEU29(0), [0]);
    assert_varint!(LEU29(127), [127]);

    assert_varint!(LEU29(128), [128, 2]);
    assert_varint!(LEU29(16383), [191, 255]);

    assert_varint!(LEU29(16384), [192, 0, 2]);
    assert_varint!(LEU29(2097151), [223, 255, 255]);

    assert_varint!(LEU29(2097152), [224, 0, 0, 1]);
    assert_varint!(LEU29(536870911), [255, 255, 255, 255]);
}

#[test]
fn test_scaler_type() {
    for word in [0x_A5C11, 0x_C0DE, 0x_DEC0DE, 0x_ADDED, 0x_AB0DE, 0x_CAFE] {
        let bytes = word.to_bytes::<LE>();
        assert_eq!(word, u32::from_bytes::<LE>(&bytes).unwrap());
    }
    for word in [
        0x_DEAD_BEEF,
        0x_Faded_Face,
        0x_BAD_F00D,
        0x_C01D_C0FFEE,
        0x_C0CA_C01A,
    ] {
        let bytes = word.to_bytes::<LE>();
        assert_eq!(word, u64::from_bytes::<LE>(&bytes).unwrap());
    }
}
