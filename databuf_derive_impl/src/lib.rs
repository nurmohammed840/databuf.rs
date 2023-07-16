mod decode;
mod encode;

pub use quote2;
pub use quote2::proc_macro2;
pub use syn;

use proc_macro2::*;
use quote2::{quote, IntoTokens, Quote, Token};
use syn::{spanned::Spanned, *};

pub fn get_enum_repr(attrs: &Vec<Attribute>) -> Option<String> {
    for Attribute { meta, .. } in attrs {
        let Meta::List(list) = meta else { continue };
        if list.path.is_ident("repr") {
            for tt in list.tokens.clone().into_iter() {
                match tt {
                    TokenTree::Ident(repr) => {
                        let repr = repr.to_string();
                        match repr.as_str() {
                            "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32"
                            | "i64" | "isize" => return Some(repr),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

pub struct Expand<'i, 'o> {
    pub crate_path: TokenStream,
    pub input: &'i DeriveInput,
    pub output: &'o mut TokenStream,
    pub enum_repr: Option<String>,
}

impl<'i, 'o> Expand<'i, 'o> {
    pub fn new(
        crate_path: TokenStream,
        input: &'i DeriveInput,
        output: &'o mut TokenStream,
    ) -> Self {
        Self {
            crate_path,
            input,
            output,
            enum_repr: get_enum_repr(&input.attrs),
        }
    }
}
