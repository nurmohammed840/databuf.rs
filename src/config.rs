/// Default Config: [num::LE] and [crate::var_int::LEU29]
pub const DEFAULT: u8 = num::LE | len::LEU30;

pub mod num {
    pub(crate) const GET: u8 = 0b111;

    // Fixed size number encoding algorithm

    /// Littel Endian
    pub const LE: u8 = 0;
    /// Big Endian
    pub const BE: u8 = 1;
    /// Native Endian
    pub const NE: u8 = 2;

    // Variable integer encoding algorithm

    /// See: https://en.wikipedia.org/wiki/LEB128
    pub const LEB128: u8 = 3;

    // /// See: https://www.rfc-editor.org/rfc/rfc9000.html#name-variable-length-integer-enc
    // pub const BEU62: u8 = 4;

    // /// See: https://en.wikipedia.org/wiki/Variable-length_quantity
    // pub const VLQ: u8 = 5;
}

// /// Negative Number encoding strategy, when used with variable integer encoding algorithms.
// /// For example `num::LEB128`
// pub mod int_codex {
//     pub const ZIG_ZAG: u8 = 0;
//     // it use sign bit to represent negetive number.
//     pub const SIGN_BIT: u8 = 1;
// }

pub mod len {
    pub(crate) const GET: u8 = 0b_11_000;

    pub const LEU30: u8 = 0b_00_000;
    pub const LEU29: u8 = 0b_01_000;

    pub const LEU22: u8 = 0b_10_000;
    pub const LEU15: u8 = 0b_11_000;
}
