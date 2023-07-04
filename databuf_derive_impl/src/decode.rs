use super::*;

pub fn expand(crate_path: &TokenStream, input: &DeriveInput, output: &mut TokenStream) {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = input;

    let body = quote(|o| {
        match data {
            Data::Struct(v) => {
                let de = decode_fields(&v.fields);
                quote!(o, { let output = Self #de });
            }
            Data::Enum(enum_data) => {
                let ident = ident.to_string();
                let ident = ident.as_str();

                let items = quote(|tokens| {
                    for (i, v) in enum_data.variants.iter().enumerate() {
                        let index = Index::from(i);
                        let name = &v.ident;
                        let fields = decode_fields(&v.fields);
                        quote!(tokens, {
                            #index => Self::#name #fields,
                        });
                    }
                });

                quote!(o, {
                    let discriminant: u16 = #crate_path::var_int::BEU15::decode::<C>(c)?.0;
                    let output = match discriminant {
                        #items
                        _ => {
                            return ::std::result::Result::Err(::std::boxed::Box::new(
                                #crate_path::error::UnknownDiscriminant {
                                    ident: ::std::concat!(::std::module_path!(), "::", #ident),
                                    discriminant,
                                },
                            ))
                        }
                    }
                });
            }
            Data::Union(_) => panic!("`Decode` implementation for `union` is not yet stabilized"),
        };
    });

    // -------------------------------------------------------------------------------------

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    // Add a bound `T: Decode<'de>` to every type parameter of `T`.
    let de_lt = Lifetime::new("'decode", Span::call_site());
    let mut lt_params = LifetimeParam::new(de_lt);
    let mut params = generics.params.clone();
    for param in params.iter_mut() {
        match param {
            GenericParam::Type(ty) => ty.bounds.push(parse_quote!(#crate_path::Decode<'decode>)),
            GenericParam::Lifetime(lt) => lt_params.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }

    quote!(output, {
        impl <#lt_params, #params> #crate_path::Decode<'decode> for #ident #ty_generics #where_clause {
            fn decode<const C: u8>(c: &mut &'decode [u8]) -> #crate_path::Result<Self> {
                use #crate_path::Decode as D;
                #body;
                ::std::result::Result::Ok(output)
            }
        }
    });
}

fn decode_fields<'a>(fields: &'a Fields) -> Token<impl FnOnce(&mut TokenStream) + 'a> {
    quote(move |o: &mut TokenStream| match fields {
        Fields::Named(fields) => {
            let fields = quote(|o| {
                for Field { ident, .. } in fields.named.iter() {
                    let de = decode_expr();
                    quote!(o, {
                        #ident: #de
                    });
                }
            });
            quote!(o, ({ #fields }));
        }
        Fields::Unnamed(fields) => {
            let de = quote(|o| {
                for _ in fields.unnamed.iter() {
                    decode_expr()(o)
                }
            });
            quote!(o, [( #de )]);
        }
        Fields::Unit => {}
    })
}

fn decode_expr() -> Token<impl Fn(&mut TokenStream)> {
    Token(|o: &mut TokenStream| {
        quote!(o, {
            D::decode::<C>(c)?,
        });
    })
}
