use crate::*;

macro_rules! impls {
    [Encoder for $($ty:ty),*] => {$(
        #[cfg(feature = "sizehint")]
        impl<L: LenType> SizeHint for Record<L, $ty> {
            #[inline] fn size_hint(&self) -> usize {
                let bytes: &[u8] = self.data.as_ref();
                std::mem::size_of::<L>() + bytes.len()
            }
        }
        impl<Len: LenType> Encoder for Record<Len, $ty>
        where
            Len: LenType,
            Len::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> {
                let len: Len = self.data.len().try_into().map_err(invalid_input)?;
                len.encoder(c)?;
                c.write_all(&self.data.as_ref())
            }
        }
        impl Encoder for $ty {
            #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> {
                let len: len::Len = self.len().try_into().map_err(invalid_input)?;
                len.encoder(c)?;
                c.write_all(&self.as_ref())
            }
        }
        // impl Encoder for $ty {
        //     #[inline] fn encoder(&self, c: &mut impl Write) -> Result<()> {
        //         Record::<Len, $ty>::encod
        //         // encode_len!(c, self.len());
        //         // c.write_all(self.as_ref())
        //     }
        // }
    )*};
}

impls!(Encoder for &[u8], &str, String);


// #[test]
// fn test_name() {
//     let data = vec![0u8; 127];
//     let re = data.as_slice();
//     // println!("{:?}", re.encode());

//     let g = Record::<len::L3, _>::new(re);
//     println!("{:?}", g.encode());
// }

// macro_rules! encode_len {
//     [$c: expr, $len: expr] => {
//         let len = $len.try_into().unwrap();
//         Len::new(len)
//             .ok_or(Error::new(ErrorKind::InvalidInput, format!("Max payload length: {}, But got {len}", Len::MAX)))?
//             .encoder($c)?;
//     }
// }
// pub(self) use encode_len;
