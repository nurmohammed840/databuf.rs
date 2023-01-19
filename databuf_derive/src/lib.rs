use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
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
                    encode_field(f, quote_spanned! {f.span()=> self.#name })
                });
                quote! { #(#recurse)* }
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(idx, f)| {
                    let index = Index::from(idx);
                    encode_field(f, quote_spanned! {f.span()=> self.#index })
                });
                quote! { #(#recurse)* }
            }
            Fields::Unit => quote! {},
        },
        Data::Enum(enum_data) => {
            let recurse = enum_data.variants.iter().enumerate().map(|(i, v)| {
                let name = &v.ident;
                let index = i as u16;
                let (fields, encode_fields) = match &v.fields {
                    Fields::Named(fields) => {
                        let field_names = fields.named.iter().enumerate().map(|(idx, f)| {
                            let old_name = &f.ident;
                            let name = format_ident!("field_{idx}");
                            quote_spanned! {f.span()=> #old_name: #name }
                        });
                        let recurse = fields
                            .named
                            .iter()
                            .enumerate()
                            .map(|(idx, f)| encode_field(f, format_ident!("field_{idx}")));

                        (quote! {  {#(#field_names),*} }, quote! { #(#recurse)* })
                    }
                    Fields::Unnamed(fields) => {
                        let field_names = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(idx, _)| format_ident!("field_{idx}"));

                        let recurse = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(idx, f)| encode_field(f, format_ident!("field_{idx}")));

                        (quote! {  (#(#field_names),*) }, quote! { #(#recurse)* })
                    }
                    Fields::Unit => (quote!(), quote!()),
                };
                quote! {
                    Self:: #name #fields => {
                        E::encoder(& #index, c)?;
                        #encode_fields
                    }
                }
            });
            quote! { match self { #(#recurse),* } }
        }
        Data::Union(_) => panic!("`Encoder` implementation for `union` is not yet stabilized"),
    };

    TokenStream::from(quote! {
        const _: () = {
            use ::databuf::Encoder as E;
            impl #impl_generics E for #ident #ty_generics #where_clause {
                fn encoder(&self, c: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                    #body
                    ::std::result::Result::Ok(())
                }
            }
        };
    })
}

fn encode_field(f: &Field, name: impl ToTokens) -> __private::TokenStream2 {
    let maybe_ref = match &f.ty {
        Type::Reference(_) => None,
        ty => Some(Token![&](ty.span())),
    };
    quote_spanned! {f.span()=> E::encoder(#maybe_ref #name, c)?;}
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
        let mut de_lifetime = LifetimeDef::new(Lifetime::new("'decoder", impl_generics.span()));
        let mut params = generics.params.clone();
        for param in &mut params {
            match param {
                GenericParam::Type(ty) => ty.bounds.push(parse_quote!(D<'decoder>)),
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
                quote! { Ok(Self { #(#recurse)* }) }
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    quote_spanned! {f.span()=>
                        D::decoder(c)?,
                    }
                });
                quote! { Ok(Self (#(#recurse)*)) }
            }
            Fields::Unit => quote! { Ok(Self) },
        },
        Data::Enum(enum_data) => {
            let recurse = enum_data.variants.iter().enumerate().map(|(i, v)| {
                let index = Index::from(i);
                let name = &v.ident;
                let body = match &v.fields {
                    Fields::Named(fields) => {
                        let fields = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote_spanned! {f.span()=> #name: D::decoder(c)? }
                        });
                        quote!({ #(#fields),* })
                    }
                    Fields::Unnamed(fields) => {
                        let fields = fields.unnamed.iter().map(|f| {
                            quote_spanned! {f.span()=> D::decoder(c)? }
                        });
                        quote! {(#(#fields),*)}
                    }
                    Fields::Unit => quote! {},
                };
                quote! { #index => Self::#name #body, }
            });
            quote! {
                let discriminant: u16 = D::decoder(c)?;
                Ok(match discriminant {
                    #(#recurse)*
                    _ => return Err(Error::from(format!("Invalid discriminant: {}", discriminant))),
                })
            }
        }
        Data::Union(_) => panic!("`Decoder` implementation for `union` is not yet stabilized"),
    };
    TokenStream::from(quote! {
        const _: () = {
            use ::databuf::{Decoder as D, Error, Result};
            use ::std::{format, result::Result::{Err, Ok}};
            impl <#lt, #ig> D<'decoder> for #ident #ty_generics #where_clause {
                fn decoder(c: &mut &'decoder [u8]) -> Result<Self> {
                    #body
                }
            }
        };
    })
}
