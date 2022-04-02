#![allow(warnings)]
use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{
    __private::{Span, TokenStream as TokenS},
    quote,
};
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use syn::*;

// -------------------------------------------------------------------------------------

#[proc_macro_derive(Encoder)]
pub fn encoder(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, generics, .. } = parse_macro_input!(input); 
    let Generics { params, where_clause, .. } = generics; 
    
    let ig = encoder_trait_bounds(params.clone());
    let (size, hint, encoder) = encoder_methods(data);

    TokenStream::from(quote! {
        impl <#ig> bin_layout::Encoder for #ident <#params> #where_clause {
            const SIZE: usize = #size;
            fn size_hint(&self) -> usize { #hint }
            fn encoder(self, c: &mut bin_layout::Cursor<impl bin_layout::Bytes>) { #encoder }
        }
    })
}

/// Add a bound `T: Encoder` to every type parameter T.
fn encoder_trait_bounds(mut params: Punctuated<GenericParam, Token![,]>) -> TokenS {
    for param in &mut params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(bin_layout::Encoder));
        }
    }
    params.to_token_stream()
}

fn encoder_methods(data: Data) -> (TokenS, TokenS, TokenS) {
    let mut size = String::from('0');
    let mut hint = String::from("use bin_layout::Encoder as S; 0");
    let mut encoder = String::from("use bin_layout::Encoder as S;");
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                for field in fields.named {
                    let name = &field.ident.unwrap();
                    write_size(&mut size, field.ty);
                    write_hint(&mut hint, &name);
                    write_encoder(&mut encoder, &name);
                }
            }
            Fields::Unnamed(fields) => {
                for (i, field) in fields.unnamed.into_iter().enumerate() {
                    write_size(&mut size, field.ty);
                    write_hint(&mut hint, i);
                    write_encoder(&mut encoder, i);
                }
            }
            Fields::Unit => {}
        },
        _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
    };
    (
        TokenS::from_str(&size).unwrap(),
        TokenS::from_str(&hint).unwrap(),
        TokenS::from_str(&encoder).unwrap(),
    )
}

fn write_size(s: &mut String, ty: Type) {
    s.push_str(" + <");
    s.push_str(&ty.to_token_stream().to_string());
    s.push_str(" as bin_layout::Encoder>::SIZE");
}
fn write_hint<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str(" + S::size_hint(&self.");
    s.push_str(&ident.to_string());
    s.push(')');
}
fn write_encoder<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str("S::encoder(self.");
    s.push_str(&ident.to_string());
    s.push_str(",c);");
}

// -------------------------------------------------------------------------------

#[proc_macro_derive(Decoder)]
pub fn decoder(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, generics, .. } = parse_macro_input!(input);
    let Generics { params, where_clause, .. } = generics;

    let (lt, ig) = decoder_trait_bounds(params.clone());
    let decoder = decoder_method(data);

    TokenStream::from(quote! {
        impl <#lt, Error: bin_layout::Error, #ig> bin_layout::Decoder<'de, Error> for #ident <#params> 
        #where_clause
        {
            fn decoder(c: &mut bin_layout::Cursor<&'de [u8]>) -> core::result::Result<Self, Error> {
                #decoder
            }
        }
    })
}

fn decoder_method(data: Data) -> TokenS {
    const DECODER: &str = "D::decoder(c)?,";
    
    let mut decoder = String::from("use bin_layout::Decoder as D; Ok(Self");
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                decoder.push('{');
                for field in fields.named {
                    decoder.push_str(&field.ident.unwrap().to_string());
                    decoder.push(':');
                    decoder.push_str(DECODER);
                }
                decoder.push('}');
            }
            Fields::Unnamed(fields) => {
                decoder.push('(');
                for _ in fields.unnamed {
                    decoder.push_str(DECODER);
                }
                decoder.push(')');
            }
            Fields::Unit => {}
        },
        _ => panic!("Default `Decoder` implementation for `enum` not yet stabilized"),
    };
    decoder.push(')');
    TokenS::from_str(&decoder).unwrap()
}

/// Add a bound `T: Decoder<'de, Error>` to every type parameter T.
fn decoder_trait_bounds(mut params: Punctuated<GenericParam, Token![,]>) -> (LifetimeDef, TokenS) {
    let mut de_lifetime = LifetimeDef::new(Lifetime::new("'de", Span::call_site()));
    for param in &mut params {
        match param {
            GenericParam::Type(ty) => ty
                .bounds
                .push(parse_quote!(bin_layout::Decoder<'de, Error>)),

            GenericParam::Lifetime(lt) => de_lifetime.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }
    (de_lifetime, params.to_token_stream())
}
