use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::*;

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
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
                let index = Index::from(i);
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
                        E::encode::<C>(&LEU15(#index), c)?;
                        #encode_fields
                    }
                }
            });
            quote! {
                use ::databuf::var_int::LEU15;
                match self { #(#recurse),* }
            }
        }
        Data::Union(_) => panic!("`Encode` implementation for `union` is not yet stabilized"),
    };

    TokenStream::from(quote! {
        impl #impl_generics ::databuf::Encode for #ident #ty_generics #where_clause {
            fn encode<const C: u8>(&self, c: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                use ::databuf::Encode as E;
                #body
                ::std::result::Result::Ok(())
            }
        }
    })
}

fn encode_field(f: &Field, name: impl ToTokens) -> __private::TokenStream2 {
    let maybe_ref = match &f.ty {
        Type::Reference(_) => None,
        ty => Some(Token![&](ty.span())),
    };
    quote_spanned! {f.span()=> E::encode::<C>(#maybe_ref #name, c)?;}
}

fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}

// -------------------------------------------------------------------------------

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = parse_macro_input!(input);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Add a bound `T: Decode<'de>` to every type parameter of `T`.
    let (lt, ig) = {
        let mut de_lifetime = LifetimeDef::new(Lifetime::new("'decode", impl_generics.span()));
        let mut params = generics.params.clone();
        for param in &mut params {
            match param {
                GenericParam::Type(ty) => ty.bounds.push(parse_quote!(D<'decode>)),
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
                        #name: D::decode::<C>(c)?,
                    }
                });
                quote! { Self { #(#recurse)* } }
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    quote_spanned! {f.span()=>
                        D::decode::<C>(c)?,
                    }
                });
                quote! { Self (#(#recurse)*) }
            }
            Fields::Unit => quote! { Self },
        },
        Data::Enum(enum_data) => {
            let recurse = enum_data.variants.iter().enumerate().map(|(i, v)| {
                let index = Index::from(i);
                let name = &v.ident;
                let body = match &v.fields {
                    Fields::Named(fields) => {
                        let fields = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote_spanned! {f.span()=> #name: D::decode::<C>(c)? }
                        });
                        quote!({ #(#fields),* })
                    }
                    Fields::Unnamed(fields) => {
                        let fields = fields.unnamed.iter().map(|f| {
                            quote_spanned! {f.span()=> D::decode::<C>(c)? }
                        });
                        quote! {(#(#fields),*)}
                    }
                    Fields::Unit => quote! {},
                };
                quote! { #index => Self::#name #body, }
            });
            let fmt_arg = format!("Unknown discriminant of `{{}}::{ident}`: {{}}");
            quote! ({
                let discriminant: u16 = ::databuf::var_int::LEU15::decode::<C>(c)?.0;
                match discriminant {
                    #(#recurse)*
                    _ => return ::std::result::Result::Err(::databuf::Error::from(::std::format!(#fmt_arg, ::std::module_path!(), discriminant))),
                }
            })
        }
        Data::Union(_) => panic!("`Decode` implementation for `union` is not yet stabilized"),
    };
    TokenStream::from(quote! {
        impl <#lt, #ig> ::databuf::Decode<'decode> for #ident #ty_generics #where_clause {
            fn decode<const C: u8>(c: &mut &'decode [u8]) -> ::databuf::Result<Self> {
                use ::databuf::Decode as D;
                ::std::result::Result::Ok(#body)
            }
        }
    })
}
