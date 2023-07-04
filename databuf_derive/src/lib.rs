use databuf_derive_impl::{
    quote2::{quote, Quote},
    *,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_macro_input;

fn crate_path() -> TokenStream2 {
    let mut o = TokenStream2::new();
    quote!(o, { ::databuf });
    o
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    encode::expand(&crate_path(), &input, &mut output);
    TokenStream::from(output)
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    decode::expand(&crate_path(), &input, &mut output);
    TokenStream::from(output)
}
