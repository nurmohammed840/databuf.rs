use databuf_derive_impl::{
    quote2::{quote, Quote},
    syn::DeriveInput,
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

fn expand<F>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(&TokenStream2, &DeriveInput, &mut TokenStream2),
{
    let input: DeriveInput = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    f(&crate_path(), &input, &mut output);
    TokenStream::from(output)
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    expand(input, encode::expand)
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    expand(input, decode::expand)
}
