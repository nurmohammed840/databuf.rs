use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

#[proc_macro_derive(Encoder)]
pub fn encoder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    add_trait_bounds(&mut generics, parse_quote!(E));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match data {
        Data::Struct(object) => match object.fields {
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
        const _: () = {
            use ::databuf::Encoder as E;
            impl #impl_generics E for #ident #ty_generics #where_clause {
                #[inline] fn encoder(&self, c: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                    #body
                    Ok(())
                }
            }
        };
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

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Add a bound `T: Decoder<'de>` to every type parameter of `T`.
    let (lt, ig) = {
        let mut de_lifetime = LifetimeDef::new(Lifetime::new("'__de__", impl_generics.span()));
        let mut params = generics.params.clone();
        for param in &mut params {
            match param {
                GenericParam::Type(ty) => ty.bounds.push(parse_quote!(D<'__de__>)),
                GenericParam::Lifetime(lt) => de_lifetime.bounds.push(lt.lifetime.clone()),
                _ => {}
            }
        }
        (de_lifetime, params)
    };

    let body = match data {
        Data::Struct(object) => match object.fields {
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
        const _: () = {
            use ::databuf::Decoder as D;
            impl <#lt, #ig> D<'__de__> for #ident #ty_generics #where_clause {
                #[inline] fn decoder(c: &mut &'__de__ [u8]) -> ::databuf::Result<Self> {
                    Ok(Self #body)
                }
            }
        };
    })
}
