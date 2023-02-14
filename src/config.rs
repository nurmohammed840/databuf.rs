/// Default Config: [num::LE] and [crate::var_int::LEU29]
pub const DEFAULT: u8 = num::LE | len::LEU29;

pub mod num {
    pub(crate) const GET: u8 = 0b111;

    // Fixed size number encoding algorithm
    /// Littel Endian
    pub const LE: u8 = 0;
    /// Big Endian
    pub const BE: u8 = 1;
    /// Native Endian
    pub const NE: u8 = 2;

    // Variable length integer encoding algorithm
    // pub const LEB128: u8 = 3;

    // /// See: https://www.rfc-editor.org/rfc/rfc9000.html#name-variable-length-integer-enc
    // pub const BEU62: u8 = 4;
}

pub mod len {
    pub(crate) const GET: u8 = 0b_11_000;

    pub const LEU29: u8 = 0b_00_000;

    pub const LEU22: u8 = 0b_01_000;
    pub const LEU15: u8 = 0b_10_000;

}
