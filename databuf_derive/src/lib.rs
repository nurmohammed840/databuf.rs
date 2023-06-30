use databuf_derive_impl::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    encode::expand(quote! { ::databuf }, &input, &mut output);
    TokenStream::from(output)
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut output = TokenStream2::new();
    decode::expand(quote! { ::databuf }, &input, &mut output);
    TokenStream::from(output)
}
