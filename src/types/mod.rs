// mod zero_copy;
// mod collection;
mod compound;
mod enumerate;
mod scalar;
mod wrapper;

// macro_rules! encode_len {
//     [$c: expr, $len: expr] => {
//         let len = $len.try_into().unwrap();
//         Len::new(len)
//             .ok_or(Error::new(ErrorKind::InvalidInput, format!("Max payload length: {}, But got {len}", Len::MAX)))?
//             .encoder($c)?;
//     }
// }
// pub(self) use encode_len;

