// #[cfg(feature = "nightly")]
// mod specialize;
use crate::*;

mod collection;
mod string;

macro_rules! encode_len {
    [$data:expr, $c: expr] => {
        let len = $data.len();
        match CONFIG & config::len::GET {
            config::len::LEU15 => {
                let len = var_int::LEU15::try_from(len).map_err(utils::invalid_input)?;
                len.encode::<CONFIG>($c)?;
            },
            config::len::LEU22 => {
                let len = var_int::LEU22::try_from(len).map_err(utils::invalid_input)?;
                len.encode::<CONFIG>($c)?;
            },
            config::len::LEU29 => {
                let len = var_int::LEU29::try_from(len).map_err(utils::invalid_input)?;
                len.encode::<CONFIG>($c)?;
            },
            _ => unreachable!()
        }
    };
}

macro_rules! decode_len {
    [$c: expr] => ({
        match CONFIG & config::len::GET {
            config::len::LEU15 => { usize::try_from(var_int::LEU15::decode::<CONFIG>($c)?)? }
            config::len::LEU22 => { usize::try_from(var_int::LEU22::decode::<CONFIG>($c)?)? }
            config::len::LEU29 => { usize::try_from(var_int::LEU29::decode::<CONFIG>($c)?)? }
            _ => unreachable!()
        }
    });
}
pub(crate) use decode_len;
pub(crate) use encode_len;
