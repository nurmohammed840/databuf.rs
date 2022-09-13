use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::*;

use quote::__private::{Span, TokenStream as TokenS};

#[proc_macro_derive(Encoder)]
pub fn encoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    trait_bounds(&mut generics, parse_quote!(bin_layout::Encoder));
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let body = {
        let mut body = String::from("use bin_layout::Encoder as _E_;");
        let mut write_encoder = |is_ref, ident: String| {
            body.push_str("_E_::encoder(");
            body.push_str(if is_ref { "self." } else { "&self." });
            body.push_str(&ident);
            body.push_str(",c)?;");
        };
        match data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    for field in fields.named {
                        let is_ref = matches!(field.ty, Type::Reference(_));
                        write_encoder(is_ref, field.ident.unwrap().to_string());
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, field) in fields.unnamed.into_iter().enumerate() {
                        let is_ref = matches!(field.ty, Type::Reference(_));
                        write_encoder(is_ref, i.to_string());
                    }
                }
                Fields::Unit => {}
            },
            _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
        };
        body.push_str("Ok(())");
        TokenS::from_str(&body).unwrap()
    };
    TokenStream::from(quote! {
        impl #generics bin_layout::Encoder for #ident #ty_generics #where_clause {
            fn encoder(&self, c: &mut impl std::io::Write) -> std::io::Result<()> { #body }
        }
    })
}

/// Add a bound `T: Encoder` to every type parameter of `T`.
fn trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
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
    let body = {
        let mut body = String::from("use bin_layout::Decoder as _D_; Ok(Self");
        match data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    body.push('{');
                    for field in fields.named {
                        body.push_str(&field.ident.unwrap().to_string());
                        body.push(':');
                        body.push_str("_D_::decoder(c)?,");
                    }
                    body.push('}');
                }
                Fields::Unnamed(fields) => {
                    body.push('(');
                    for _ in fields.unnamed {
                        body.push_str("_D_::decoder(c)?,");
                    }
                    body.push(')');
                }
                Fields::Unit => {}
            },
            _ => panic!("Default `Decoder<'_>` implementation for `enum` not yet stabilized"),
        };
        body.push(')');
        TokenS::from_str(&body).unwrap()
    };
    TokenStream::from(quote! {
        impl <#lt, #ig> bin_layout::Decoder<'_de_> for #ident #ty_generics
        #where_clause
        {
            fn decoder(c: &mut &'_de_ [u8]) -> std::io::Result<Self> {
                #body
            }
        }
    })
}

/// Add a bound `T: Decoder<'de>` to every type parameter of `T`.
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