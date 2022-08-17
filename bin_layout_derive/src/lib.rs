use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::*;

use quote::__private::{Span, TokenStream as TokenS};

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
    let (hint, encoder) = encoder_method(data);

    TokenStream::from(quote! {
        impl #mod_generics bin_layout::Encoder for #ident #ty_generics #where_clause {
            fn size_hint(&self) -> usize { #hint }
            fn encoder(&self, c: &mut impl std::io::Write) -> std::io::Result<()> { #encoder }
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
fn encoder_method(data: Data) -> (TokenS, TokenS) {
    let mut hint = String::from("use bin_layout::Encoder as _E_; 0");
    let mut encoder = String::from("use bin_layout::Encoder as _E_;");
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                for field in fields.named {
                    let name = &field.ident.unwrap();
                    write_hint(&mut hint, &name);
                    write_encoder(&mut encoder, &name);
                }
            }
            Fields::Unnamed(fields) => {
                for (i, _) in fields.unnamed.iter().enumerate() {
                    write_hint(&mut hint, i);
                    write_encoder(&mut encoder, i);
                }
            }
            Fields::Unit => {}
        },
        _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
    };
    encoder.push_str("Ok(())");
    (
        TokenS::from_str(&hint).unwrap(),
        TokenS::from_str(&encoder).unwrap(),
    )
}
fn write_hint<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str(" + _E_::size_hint(&self.");
    s.push_str(&ident.to_string());
    s.push(')');
}
fn write_encoder<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str("_E_::encoder(&self.");
    s.push_str(&ident.to_string());
    s.push_str(",c)?;");
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
        impl <#lt, #ig> bin_layout::Decoder<'_de_> for #ident #ty_generics
        #where_clause
        {
            fn decoder(c: &mut &'_de_ [u8]) -> std::io::Result<Self> {
                #decoder
            }
        }
    })
}
fn decoder_method(data: Data) -> TokenS {
    let mut decoder = String::from("use bin_layout::Decoder as _D_; Ok(Self");
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                decoder.push('{');
                for field in fields.named {
                    decoder.push_str(&field.ident.unwrap().to_string());
                    decoder.push(':');
                    decoder.push_str("_D_::decoder(c)?,");
                }
                decoder.push('}');
            }
            Fields::Unnamed(fields) => {
                decoder.push('(');
                for _ in fields.unnamed {
                    decoder.push_str("_D_::decoder(c)?,");
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

/// Add a bound `T: Decoder<'de>` to every type parameter T.
fn decoder_trait_bounds(g: &Generics) -> (LifetimeDef, Punctuated<GenericParam, token::Comma>) {
    let mut de_lifetime = LifetimeDef::new(Lifetime::new("'_de_", Span::call_site()));
    let mut params = g.params.clone();
    for param in &mut params {
        match param {
            GenericParam::Type(ty) => ty.bounds.push(parse_quote!(bin_layout::Decoder<'_de_>)),

            GenericParam::Lifetime(lt) => de_lifetime.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }
    (de_lifetime, params)
}
