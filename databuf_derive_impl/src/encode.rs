use super::*;
use quote::quote;

pub fn expand(crate_path: impl ToTokens, input: &DeriveInput, mut output: &mut TokenStream) {
    let DeriveInput {
        data,
        ident,
        generics,
        ..
    } = input;

    let mut body = TokenStream::new();
    match data {
        Data::Struct(object) => match &object.fields {
            Fields::Named(fields) => fields.named.iter().for_each(|f| {
                encode_field(f, self_field(&f.ident), &mut body);
            }),
            Fields::Unnamed(fields) => fields.unnamed.iter().enumerate().for_each(|(idx, f)| {
                encode_field(f, self_field(Index::from(idx)), &mut body);
            }),
            Fields::Unit => {}
        },
        Data::Enum(enum_data) => {
            quote_each_token! {body
                use ::databuf::var_int::BEU15;
                match self
            }
            group!(body, Brace, tokens, {
                for (i, v) in enum_data.variants.iter().enumerate() {
                    let named = &v.ident;
                    let index = Index::from(i);

                    let mut encode_fields = TokenStream::new();

                    let struct_fields = match &v.fields {
                        Fields::Named(fields) => Some(Group::new(Delimiter::Brace, {
                            let mut names = TokenStream::new();
                            for (i, f) in fields.named.iter().enumerate() {
                                let alias = Ident::new(&format!("_{i}"), Span::call_site());
                                encode_field(f, &alias, &mut encode_fields);
                                names.append_all(&f.ident);
                                names.append(Punct::new(':', Spacing::Alone));
                                names.append(alias);
                            }
                            names
                        })),
                        Fields::Unnamed(fields) => Some(Group::new(Delimiter::Parenthesis, {
                            let mut names = TokenStream::new();
                            for (i, f) in fields.unnamed.iter().enumerate() {
                                let name = Ident::new(&format!("_{i}"), Span::call_site());
                                encode_field(f, &name, &mut encode_fields);
                                names.append(name);
                            }
                            names
                        })),
                        Fields::Unit => None,
                    };
                    quote_each_token! {tokens
                        Self:: #named #struct_fields =>
                    }
                    group!(tokens, Brace, s, {
                        quote_each_token! {s
                            E::encode::<C>(&BEU15(#index), c)?;
                        }
                        s.extend(encode_fields);
                    });
                }
            });
        }
        Data::Union(_) => panic!("`Encode` implementation for `union` is not yet stabilized"),
    };

    // ------------------------------------------------------------------------------------------

    let encode_trait = quote!(#crate_path::Encode);

    let mut generics = generics.clone();
    add_trait_bounds(&mut generics, parse_quote(encode_trait.clone()));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote_each_token! {output impl #impl_generics #encode_trait for #ident #ty_generics #where_clause };
    group!(output, Brace, s, {
        quote_each_token! {s fn encode<const C: u8>(&self, c: &mut impl ::std::io::Write) -> ::std::io::Result<()> };
        group!(s, Brace, s, {
            quote_each_token! {s  use #encode_trait as E; }
            s.extend(body);
            quote_each_token! {s  ::std::result::Result::Ok(()) }
        });
    });
}

fn self_field(name: impl ToTokens) -> TokenStream {
    quote! { self.#name }
}

fn encode_field(f: &Field, name: impl ToTokens, mut tokens: &mut TokenStream) {
    let maybe_ref = match &f.ty {
        Type::Reference(_) => None,
        ty => Some(Token![&](ty.span())),
    };
    quote_each_token! {tokens
        E::encode::<C>(#maybe_ref #name, c)?;
    }
}

fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}
