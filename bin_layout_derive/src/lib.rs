use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::*;

use quote::__private::{Span, TokenStream as TokenS};
use syn::__private::ToTokens;

// -------------------------------------------------------------------------------------

#[proc_macro_derive(Encoder)]
pub fn encoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = parse_macro_input!(input);

    let mod_generics = encoder_trait_bounds(generics.clone());
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let (size, hint, encoder) = encoder_methods(data);

    TokenStream::from(quote! {
        impl #mod_generics bin_layout::Encoder for #ident #ty_generics #where_clause {
            const SIZE: usize = #size;
            fn size_hint(&self) -> usize { #hint }
            fn encoder(self, c: &mut bin_layout::Cursor<impl bin_layout::Bytes>) { #encoder }
        }
    })
}
/// Add a bound `T: Encoder` to every type parameter T.
fn encoder_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(bin_layout::Encoder));
        }
    }
    generics
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
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = parse_macro_input!(input);

    let (lt, ig) = decoder_trait_bounds(&generics);
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let decoder = decoder_method(data);

    TokenStream::from(quote! {
        impl <#lt, #ig> bin_layout::Decoder<'de, Error> for #ident #ty_generics
        #where_clause
        {
            fn decoder(c: &mut bin_layout::Cursor<&'de [u8]>) -> core::result::Result<Self, Error> {
                #decoder
            }
        }
    })
}
fn decoder_method(data: Data) -> TokenS {
    let mut decoder = String::from("use bin_layout::Decoder as D; Ok(Self");
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                decoder.push('{');
                for field in fields.named {
                    decoder.push_str(&field.ident.unwrap().to_string());
                    decoder.push(':');
                    decoder.push_str("D::decoder(c)?,");
                }
                decoder.push('}');
            }
            Fields::Unnamed(fields) => {
                decoder.push('(');
                for _ in fields.unnamed {
                    decoder.push_str("D::decoder(c)?,");
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
fn decoder_trait_bounds(g: &Generics) -> (LifetimeDef, Punctuated<GenericParam, token::Comma>) {
    let mut de_lifetime = LifetimeDef::new(Lifetime::new("'de", Span::call_site()));
    let mut params = g.params.clone();
    for param in &mut params {
        match param {
            GenericParam::Type(ty) => ty
                .bounds
                .push(parse_quote!(bin_layout::Decoder<'de, Error>)),

            GenericParam::Lifetime(lt) => de_lifetime.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }
    let i = params
        .iter()
        .enumerate()
        .find_map(|(i, g)| {
            matches!(g, GenericParam::Type(..) | GenericParam::Const(..)).then(|| i)
        })
        .unwrap_or(params.len());

    params.insert(
        i,
        GenericParam::Type(parse_quote!(Error: bin_layout::Error)),
    );

    (de_lifetime, params)
}
