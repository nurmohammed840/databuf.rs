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
                if let TokenTree::Ident(repr) = tt {
                    let repr = repr.to_string();
                    if repr.as_str().starts_with(['i', 'u']) {
                        return Some(repr);
                    }
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
    pub is_unit_enum: bool,
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
            is_unit_enum: {
                if let Data::Enum(data) = &input.data {
                    data.variants
                        .iter()
                        .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit))
                } else {
                    false
                }
            },
        }
    }
}

struct Discriminator {
    discriminant: Index,
    expr: Option<Expr>,
    is_decoder: bool,
}

impl Discriminator {
    fn new(is_decoder: bool) -> Self {
        Self {
            discriminant: Index::from(0),
            expr: None,
            is_decoder,
        }
    }
    fn get<'a>(
        &'a mut self,
        discriminant: &'a Option<(Token!(=), Expr)>,
    ) -> Token<impl FnOnce(&mut TokenStream) + 'a> {
        quote(move |o| match discriminant {
            Some((_, expr)) => {
                self.discriminant.index = 1;
                self.expr = Some(expr.clone());
                quote!(o, { #expr });
            }
            None => {
                let index = &self.discriminant;
                if let Some(expr) = &self.expr {
                    if self.is_decoder {
                        quote!(o, { v if v == });
                    }
                    quote!(o, { #expr + #index });
                } else {
                    quote!(o, { #index });
                }
                self.discriminant.index += 1;
            }
        })
    }
}
