use super::*;
use syn::punctuated::Iter;

pub fn expand(crate_path: &TokenStream, input: &DeriveInput, o: &mut TokenStream) {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = input;

    let body = quote(|o| {
        match data {
            Data::Struct(object) => match &object.fields {
                Fields::Named(fields) => fields.named.iter().for_each(|f| {
                    encode_field(f, field(&f.ident), o);
                }),
                Fields::Unnamed(fields) => {
                    fields.unnamed.iter().enumerate().for_each(|(idx, f)| {
                        encode_field(f, field(Index::from(idx)), o);
                    })
                }
                Fields::Unit => {}
            },
            Data::Enum(enum_data) => {
                let items = quote(|o| {
                    for (i, v) in enum_data.variants.iter().enumerate() {
                        let named = &v.ident;
                        let index = Index::from(i);
                        let mut encoders = Token(TokenStream::new());

                        let alias = quote(|o| {
                            match &v.fields {
                                Fields::Named(f) => {
                                    let alias = make_alias(true, f.named.iter(), &mut encoders);
                                    quote!(o, {{ #alias }});
                                }
                                Fields::Unnamed(f) => {
                                    let alias = make_alias(false, f.unnamed.iter(), &mut encoders);
                                    quote!(o, {( #alias )});
                                }
                                Fields::Unit => {}
                            };
                        });
                        quote!(o, {
                            Self:: #named #alias => {
                                E::encode::<C>(&BEU15(#index), c)?;
                                #encoders
                            }
                        });
                    }
                });
                quote!(o, {
                    use ::databuf::var_int::BEU15;
                    match self {
                        #items
                    }
                });
            }
            Data::Union(_) => panic!("`Encode` implementation for `union` is not yet stabilized"),
        };
    });

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let bound: TypeParamBound = parse_quote!(#crate_path::Encode);
    let mut params = generics.params.clone();

    for param in params.iter_mut() {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(bound.clone())
        }
    }

    quote!(o, {
        impl<#params> #crate_path::Encode for #ident #ty_generics #where_clause {
            fn encode<const C: u8>(&self, c: &mut (impl ::std::io::Write + ?::std::marker::Sized)) -> ::std::io::Result<()> {
                use #crate_path::Encode as E;
                #body
                ::std::result::Result::Ok(())
            }
        }
    });
}

fn make_alias<'a>(
    is_named: bool,
    fields: Iter<'a, Field>,
    encoders: &'a mut TokenStream,
) -> Token<impl FnOnce(&mut TokenStream) + 'a> {
    quote(move |o| {
        for (i, f) in fields.enumerate() {
            let alias = Ident::new(&format!("_{i}"), Span::call_site());
            encode_field(f, &alias, encoders);
            if is_named {
                let name = &f.ident;
                quote!(o, {
                    #name: #alias,
                });
            } else {
                quote!(o, { #alias, });
            }
        }
    })
}

fn field(name: impl IntoTokens) -> Token<impl FnOnce(&mut TokenStream)> {
    quote(move |o| {
        quote!(o, { self.#name });
    })
}

fn encode_field(f: &Field, field: impl IntoTokens, o: &mut TokenStream) {
    let maybe_ref = match &f.ty {
        Type::Reference(_) => None,
        ty => Some(Token![&](ty.span())),
    };
    quote!(o, {
        E::encode::<C>(#maybe_ref #field, c)?;
    });
}
