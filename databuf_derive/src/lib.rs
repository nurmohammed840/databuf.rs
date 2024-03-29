use databuf_derive_impl::{
    quote2::{quote, Quote},
    syn::DeriveInput,
    Expand, *,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_macro_input;

fn expand<F>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(Expand),
{
    let input: DeriveInput = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    let mut crate_path = TokenStream2::new();

    quote!(crate_path, { ::databuf });
    f(Expand::new(crate_path, &input, &mut output));
    TokenStream::from(output)
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    expand(input, |mut expend| expend.encoder())
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    expand(input, |mut expend| expend.decoder())
}
