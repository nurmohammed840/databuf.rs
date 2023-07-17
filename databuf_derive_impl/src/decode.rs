use super::*;

impl Expand<'_, '_> {
    pub fn decoder(&mut self) {
        let crate_path = &self.crate_path;
        let enum_repr = &self.enum_repr;
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
                Data::Struct(v) => {
                    let de = decode_fields(&v.fields);
                    quote!(o, { let output = Self #de });
                }
                Data::Enum(enum_data) => {
                    let items = quote(|o| {
                        let mut discriminator = Discriminator::new(true);
                        for Variant {
                            ident,
                            fields,
                            discriminant,
                            ..
                        } in enum_data.variants.iter()
                        {
                            let index = discriminator.get(discriminant);
                            let fields = decode_fields(fields);
                            quote!(o, {
                                #index => Self::#ident #fields,
                            });
                        }
                    });

                    let id = quote(|o| {
                        let ty = match enum_repr {
                            Some(repr) => repr,
                            None if !is_unit_enum => {
                                quote!(o, {
                                    let discriminant: u16 = #crate_path::var_int::BEU15::decode::<C>(c)?.0;
                                });
                                return;
                            }
                            None => "isize",
                        };
                        let repr = Ident::new(ty, Span::call_site());
                        quote!(o, {
                            let discriminant: #repr = D::decode::<C>(c)?;
                        });
                    });

                    let ident = ident.to_string();
                    quote!(o, {
                        #id
                        let output = match discriminant {
                            #items
                            _ => {
                                return #crate_path::error::UnknownDiscriminant::new_boxed_err(
                                    ::std::concat!(::std::module_path!(), "::", #ident),
                                    discriminant
                                )
                            }
                        }
                    });
                }
                Data::Union(_) => {
                    panic!("`Decode` implementation for `union` is not yet stabilized")
                }
            };
        });

        let (_, ty_generics, where_clause) = generics.split_for_impl();

        // Add a bound `T: Decode<'de>` to every type parameter of `T`.
        let bound: TypeParamBound = parse_quote!(#crate_path::Decode<'decode>);
        let mut params = generics.params.clone();
        let mut lifetime = LifetimeParam::new(Lifetime::new("'decode", Span::call_site()));

        for param in params.iter_mut() {
            match param {
                GenericParam::Type(ty) => ty.bounds.push(bound.clone()),
                GenericParam::Lifetime(lt) => lifetime.bounds.push(lt.lifetime.clone()),
                _ => {}
            }
        }

        quote!(output, {
            impl <#lifetime, #params> #crate_path::Decode<'decode> for #ident #ty_generics #where_clause {
                fn decode<const C: u16>(c: &mut &'decode [u8]) -> #crate_path::Result<Self> {
                    use #crate_path::Decode as D;
                    #body;
                    ::std::result::Result::Ok(output)
                }
            }
        });
    }
}

fn decode_fields(fields: &Fields) -> Token<impl FnOnce(&mut TokenStream) + '_> {
    let expr = quote(|o| {
        quote!(o, {
            D::decode::<C>(c)?,
        });
    });
    quote(move |o: &mut TokenStream| match fields {
        Fields::Named(fields) => {
            let fields = quote(|o| {
                for Field { ident, .. } in fields.named.iter() {
                    quote!(o, { #ident: #expr });
                }
            });
            quote!(o, {{ #fields }});
        }
        Fields::Unnamed(fields) => {
            let de = quote(|o| fields.unnamed.iter().for_each(|_| expr(o)));
            quote!(o, {( #de )});
        }
        Fields::Unit => {}
    })
}
