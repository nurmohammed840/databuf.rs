use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::*;

use quote::__private::{Span, TokenStream as TokenS};

#[cfg(feature = "sizehint")]
#[proc_macro_derive(SizeHint)]
pub fn size_hint(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);
    // Add a bound `T: SizeHint` to every type parameter T.
    trait_bounds(&mut generics, parse_quote!(bin_layout::SizeHint));
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let body = {
        let mut body = String::from("use bin_layout::SizeHint as _SH_; 0");
        let mut write_hint = |ident: String| {
            body.push_str(" + _SH_::size_hint(&self.");
            body.push_str(&ident);
            body.push(')');
        };
        match data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    for field in fields.named {
                        write_hint(field.ident.unwrap().to_string());
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        write_hint(i.to_string());
                    }
                }
                Fields::Unit => {}
            },
            _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
        };
        TokenS::from_str(&body).unwrap()
    };

    TokenStream::from(quote! {
        impl #generics bin_layout::SizeHint for #ident #ty_generics #where_clause {
            fn size_hint(&self) -> usize { #body }
        }
    })
}

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
        let mut write_encoder = |ident:String| {
            body.push_str("_E_::encoder(&self.");
            body.push_str(&ident);
            body.push_str(",c)?;");
        };
        match data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    for field in fields.named {
                        write_encoder(field.ident.unwrap().to_string());
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        write_encoder(i.to_string());
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
    let body = decoder_body(data);

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
fn decoder_body(data: Data) -> TokenS {
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
        _ => panic!("Default `Decoder` implementation for `enum` not yet stabilized"),
    };
    body.push(')');
    TokenS::from_str(&body).unwrap()
}
