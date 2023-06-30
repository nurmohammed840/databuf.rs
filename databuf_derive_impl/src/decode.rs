use super::*;

pub fn expand(crate_path: impl ToTokens, input: &DeriveInput, mut output: &mut TokenStream) {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = input;

    let mut body = TokenStream::new();

    match data {
        Data::Struct(v) => {
            quote_each_token! {body let output = Self }
            decode_fields(&v.fields, &mut body);
        }
        Data::Enum(enum_data) => {
            let ident = ident.to_string();
            quote_each_token! {body
                let discriminant: u16 = #crate_path::var_int::BEU15::decode::<C>(c)?.0;
                let output = match discriminant
            };
            group!(body, Brace, tokens, {
                for (i, v) in enum_data.variants.iter().enumerate() {
                    let name = &v.ident;
                    let index = Index::from(i);

                    quote_each_token! {tokens
                        #index => Self::#name
                    }
                    decode_fields(&v.fields, &mut tokens);
                    tokens.append(Punct::new(',', Spacing::Alone));
                }
                quote_each_token! {tokens
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
            // quote_each_token! {body  };
        }
        Data::Union(_) => panic!("`Decode` implementation for `union` is not yet stabilized"),
    };

    // -------------------------------------------------------------------------------------

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    // Add a bound `T: Decode<'de>` to every type parameter of `T`.
    let mut lt_params = LifetimeParam::new(Lifetime::new("'decode", Span::call_site()));
    let mut params = generics.params.clone();
    for param in &mut params {
        match param {
            GenericParam::Type(ty) => ty.bounds.push(parse_quote!(#crate_path::Decode<'decode>)),
            GenericParam::Lifetime(lt) => lt_params.bounds.push(lt.lifetime.clone()),
            _ => {}
        }
    }

    quote_each_token! {output
        impl <#lt_params, #params> #crate_path::Decode<'decode> for #ident #ty_generics #where_clause
    }
    group!(output, Brace, s, {
        quote_each_token! {s fn decode<const C: u8>(c: &mut &'decode [u8]) -> #crate_path::Result<Self> }
        group!(s, Brace, s, {
            quote_each_token! {s use #crate_path::Decode as D; }
            s.extend(body);
            quote_each_token! {s ; ::std::result::Result::Ok(output) }
        });
    });
}

fn decode_fields(fields: &Fields, tokens: &mut TokenStream) {
    match fields {
        Fields::Named(fields) => {
            group!(tokens, Brace, s, {
                for field in fields.named.iter() {
                    s.append_all(&field.ident);
                    s.append(Punct::new(':', Spacing::Alone));
                    decode_expr(&mut s);
                }
            });
        }
        Fields::Unnamed(fields) => {
            group!(tokens, Parenthesis, s, {
                for _ in fields.unnamed.iter() {
                    decode_expr(&mut s);
                }
            });
        }
        Fields::Unit => {}
    }
}

fn decode_expr(mut tokens: &mut TokenStream) {
    quote_each_token! {tokens D::decode::<C>(c)?, }
}
