use super::*;
use syn::punctuated::Iter;

impl Expand<'_, '_> {
    pub fn encoder(&mut self) {
        let crate_path = &self.crate_path;
        let enum_repr = self.enum_repr.as_ref();
        let is_unit_enum = &self.is_unit_enum;
        let output = &mut self.output;
        let DeriveInput {
            data,
            ident,
            generics,
            ..
        } = self.input;

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
                        let mut discriminator = Discriminator::new(false);

                        for Variant {
                            ident,
                            fields,
                            discriminant,
                            ..
                        } in &enum_data.variants
                        {
                            let index = discriminator.get(discriminant);
                            let mut encoders = Token(TokenStream::new());

                            let alias = quote(|o| {
                                match &fields {
                                    Fields::Named(f) => {
                                        let alias = make_alias(true, f.named.iter(), &mut encoders);
                                        quote!(o, {{ #alias }});
                                    }
                                    Fields::Unnamed(f) => {
                                        let alias =
                                            make_alias(false, f.unnamed.iter(), &mut encoders);
                                        quote!(o, {( #alias )});
                                    }
                                    Fields::Unit => {}
                                };
                            });
                            let encode_index = quote(|o| {
                                let ty = match enum_repr {
                                    None if !is_unit_enum => {
                                        quote!(o, {
                                            E::encode::<C>(&BEU15(#index), c)?;
                                        });
                                        return;
                                    }
                                    Some(repr) => repr,
                                    None => "isize",
                                };
                                let repr = Ident::new(ty, Span::call_site());
                                quote!(o, {
                                    #repr::encode::<C>(&(#index), c)?;
                                });
                            });
                            quote!(o, {
                                Self:: #ident #alias => {
                                    #encode_index
                                    #encoders
                                }
                            });
                        }
                    });
                    if !is_unit_enum && enum_repr.is_none() {
                        quote!(o, {
                            use #crate_path::var_int::BEU15;
                        });
                    }
                    quote!(o, {
                        match self {
                            #items
                        }
                    });
                }
                Data::Union(_) => {
                    panic!("`Encode` implementation for `union` is not yet stabilized")
                }
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

        quote!(output, {
            impl<#params> #crate_path::Encode for #ident #ty_generics #where_clause {
                fn encode<const C: u16>(&self, c: &mut (impl ::std::io::Write + ?::std::marker::Sized)) -> ::std::io::Result<()> {
                    use #crate_path::Encode as E;
                    #body
                    ::std::result::Result::Ok(())
                }
            }
        });
    }
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
