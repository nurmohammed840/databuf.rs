use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::*;
use syn::{punctuated::Punctuated, spanned::Spanned};

#[proc_macro_derive(Encoder)]
pub fn encoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    add_trait_bounds(&mut generics, parse_quote!(databuf::Encoder));
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let body = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ref_and = match &f.ty {
                        Type::Reference(_) => None,
                        ty => Some(Token![&](ty.span())),
                    };
                    quote_spanned! {f.span()=>
                        E::encoder(#ref_and self.#name, c)?;
                    }
                });
                quote! { #(#recurse)* }
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    let ref_and = match &f.ty {
                        Type::Reference(_) => None,
                        ty => Some(Token![&](ty.span())),
                    };
                    quote_spanned! {f.span()=>
                        E::encoder(#ref_and self.#index, c)?;
                    }
                });
                quote! { #(#recurse)* }
            }
            Fields::Unit => quote! {},
        },
        _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
    };

    TokenStream::from(quote! {
        impl #generics databuf::Encoder for #ident #ty_generics #where_clause {
            #[inline] fn encoder(&self, c: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                use databuf::Encoder as E;
                #body
                Ok(())
            }
        }
    })
}

fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
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

    let (lt, ig) = add_decoder_trait_bounds(&generics);
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let body = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        #name: D::decoder(c)?,
                    }
                });
                quote!({ #(#recurse)* })
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    quote_spanned! {f.span()=>
                        D::decoder(c)?,
                    }
                });
                quote! {(#(#recurse)*)}
            }
            Fields::Unit => quote! {},
        },
        _ => panic!("Default `Decoder` implementation for `enum` not yet stabilized"),
    };
    TokenStream::from(quote! {
        impl <#lt, #ig> databuf::Decoder<'__de__> for #ident #ty_generics
        #where_clause
        {
            #[inline] fn decoder(c: &mut &'__de__ [u8]) -> databuf::Result<Self> {
                use databuf::Decoder as D;
                Ok(Self #body)
            }
        }
    })
}

/// Add a bound `T: Decoder<'de>` to every type parameter of `T`.
fn add_decoder_trait_bounds(g: &Generics) -> (LifetimeDef, Punctuated<GenericParam, token::Comma>) {
    let mut de_lifetime = LifetimeDef::new(Lifetime::new("'__de__", g.span()));
    let mut params = g.params.clone();
    for param in &mut params {
        match param {
            GenericParam::Type(ty) => ty.bounds.push(parse_quote!(databuf::Decoder<'__de__>)),
            GenericParam::Lifetime(lt) => de_lifetime.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }
    (de_lifetime, params)
}
