#![allow(clippy::unusual_byte_groupings)]

/// Default configuration:
///
/// - Numbers are represented in little endian byte order
/// - Length of the collection is encoded with [crate::var_int::BEU30]
pub const DEFAULT: u16 = num::LE | len::BEU30;

/// Configuration options for number representation
pub mod num {
    pub(crate) const GET: u16 = 0b1111;

    // Fixed size number encoding algorithm

    /// Represents numbers with little endian byte order
    pub const LE: u16 = 0;
    /// Represents numbers with big endian byte order
    pub const BE: u16 = 1;
    /// Represents numbers with native endian byte order
    pub const NE: u16 = 2;

    /// [LEB128](https://en.wikipedia.org/wiki/LEB128) or Little Endian Base 128 is a variable-length code
    /// compression used to store arbitrarily large integers in a small number of bytes.
    pub const LEB128: u16 = 3;

    // TODO: Not implemented yet...
    // See: https://www.rfc-editor.org/rfc/rfc9000.html#name-variable-length-integer-enc
    // pub const BEU62: u16 = 4;

    // See: https://en.wikipedia.org/wiki/Variable-length_quantity
    // pub const VLQ: u16 = 5;
}

// Negative Number encoding strategy, Used with variable integer encoding algorithms.
// pub mod int_codec {
//     pub const ZIG_ZAG: u16 = ...;
//     // it use sign bit to represent negetive number.
//     pub const SIGN_BIT: u16 = ...;
// }

/// Configuration options for representing length of collection.
pub mod len {
    pub(crate) const GET: u16 = 0b_111_0000;

    /// length is represented with [crate::var_int::BEU30] big-endian unsigned 30-bit integer.
    pub const BEU30: u16 = 0 << 4;

    /// length is represented with [crate::var_int::BEU29] big-endian unsigned 29-bit integer.
    pub const BEU29: u16 = 1 << 4;

    /// length is represented with [crate::var_int::BEU22] big-endian unsigned 22-bit integer.
    pub const BEU22: u16 = 2 << 4;

    /// length is represented with [crate::var_int::BEU15] big-endian unsigned 15-bit integer.
    pub const BEU15: u16 = 3 << 4;
}
