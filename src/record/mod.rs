// #[cfg(feature = "nightly")]
// mod specialize;
use crate::*;

mod collection;
mod string;

macro_rules! encode_len {
    [$data:expr, $c: expr] => {
        let len = $data.len();
        match CONFIG & config::len::GET {
            config::len::BEU30 => var_int::BEU30::try_from(len).map_err(utils::invalid_input)?.encode::<CONFIG>($c)?,
            config::len::BEU29 => var_int::BEU29::try_from(len).map_err(utils::invalid_input)?.encode::<CONFIG>($c)?,
            config::len::BEU22 => var_int::BEU22::try_from(len).map_err(utils::invalid_input)?.encode::<CONFIG>($c)?,
            config::len::BEU15 => var_int::BEU15::try_from(len).map_err(utils::invalid_input)?.encode::<CONFIG>($c)?,
            _ => unreachable!()
        }
    };
}
macro_rules! decode_len {
    [$c: expr] => {
        match CONFIG & config::len::GET {
            config::len::BEU30 => { usize::try_from(var_int::BEU30::decode::<CONFIG>($c)?)? }
            config::len::BEU29 => { usize::try_from(var_int::BEU29::decode::<CONFIG>($c)?)? }
            config::len::BEU22 => { usize::try_from(var_int::BEU22::decode::<CONFIG>($c)?)? }
            config::len::BEU15 => { usize::try_from(var_int::BEU15::decode::<CONFIG>($c)?)? }
            _ => unreachable!()
        }
    };
}
pub(crate) use decode_len;
pub(crate) use encode_len;
