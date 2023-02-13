#![allow(warnings)]
use std::default;

#[derive(Default)]
pub enum Endian {
    Native = 0,
    Big = 1,
    #[default]
    Littel = 2,
}

#[derive(Default)]
pub enum VarLen {
    #[default]
    U22 = 0b_0_00,
    U15 = 0b_1_00,
}

// TODO: I feel lazy to implement...

// #[derive(Default)]
// pub enum IntEncoder {
//     #[default]
//     Fix = 0b_0_000_00,
//     LEB128 = 0b_1_000_00,
//     VLQ = 0b_10_000_00,
//     ZigZag = 0b_11_000_00,
// }

pub struct Config(u8);

impl Config {
    const fn new() -> Self {
        Self(0)
    }
    const fn set_endian(mut self, endian: Endian) -> Self {
        self.0 |= endian as u8;
        self
    }
    const fn set_var_len(mut self, var_len: VarLen) -> Self {
        self.0 |= var_len as u8;
        self
    }
    // const fn set_int_encoder(mut self, int_encoder: IntEncoder) -> Self {
    //     self.0 |= int_encoder as u8;
    //     self
    // }
    const fn build(self) -> u8 {
        self.0
    }
}