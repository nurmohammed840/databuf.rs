#![allow(warnings)]
pub mod decode;
pub mod encode;

pub use quote2::proc_macro2;
pub use syn;

use proc_macro2::*;
use quote2::{quote, Quote};
use syn::{__private::parse_quote, spanned::Spanned, *};
