use virtue::prelude::*;

#[proc_macro_derive(DataType)]
pub fn layout(input: TokenStream) -> TokenStream {
    derive_inner(input).unwrap_or_else(|err| err.into_token_stream())
}

fn derive_inner(input: TokenStream) -> Result<TokenStream> {
    let (mut generator, _attrs, body) = Parse::new(input)?.into_generator();
    match body {
        Body::Struct(sbody) => {
            let mut gen = generator.impl_for_with_lifetimes("DataType", ["de"]);

            let mut size = String::from('0');
            let mut is_dy = String::from("true");
            let mut hint = String::from("use DataType as D; 0");
            let mut ser = String::from("use DataType as D;");
            let mut de = String::from("use DataType as D; Ok(Self");

            match sbody.fields {
                Fields::Struct(fields) => {
                    let len = fields.len();
                    size.reserve(len * 45);
                    is_dy.reserve(len * 55);
                    hint.reserve(len * 45);
                    ser.reserve(len * 45);
                    de.reserve(len * 45);

                    de.push('{');
                    for (ident, f) in fields.into_iter() {
                        write_size(&mut size, f.r#type.clone());
                        write_is_dy(&mut is_dy, f.r#type);
                        write_size_hint(&mut hint, &ident);
                        write_ser(&mut ser, &ident);
                        write_de(&mut de, ident);
                    }
                    de.push('}');
                }
                Fields::Tuple(fields) => {
                    let len = fields.len();
                    size.reserve(len * 40);
                    is_dy.reserve(len * 50);
                    hint.reserve(len * 30);
                    ser.reserve(len * 30);
                    de.reserve(len * 25);

                    de.push('(');
                    for (i, f) in fields.into_iter().enumerate() {
                        write_size(&mut size, f.r#type.clone());
                        write_is_dy(&mut is_dy, f.r#type);
                        write_size_hint(&mut hint, i);
                        write_ser(&mut ser, i);
                        de.push_str(DESERIALIZE);
                    }
                    de.push(')');
                }
                Fields::Unit => {}
                _ => unimplemented!(),
            };
            de.push(')');

            gen.generate_const("SIZE", "usize").with_value(|b| {
                b.push_parsed(size)?;
                Ok(())
            })?;

            gen.generate_const("IS_DYNAMIC", "bool").with_value(|b| {
                b.push_parsed(is_dy)?;
                Ok(())
            })?;

            gen.generate_fn("size_hint")
                .with_self_arg(FnSelfArg::RefSelf)
                .with_return_type("usize")
                .body(|b| {
                    b.push_parsed(hint)?;
                    Ok(())
                })?;

            gen.generate_fn("serialize")
                .with_self_arg(FnSelfArg::TakeSelf)
                .with_arg("c", "&mut bin_layout::Cursor<impl bin_layout::Bytes>")
                .body(|fn_body| {
                    fn_body.push_parsed(ser)?;
                    Ok(())
                })?;

            gen.generate_fn("deserialize")
                .with_arg("c", "&mut bin_layout::Cursor<&'de [u8]>")
                .with_return_type("bin_layout::Result<Self>")
                .body(|fn_body| {
                    fn_body.push_parsed(de)?;
                    Ok(())
                })?;
        }
        Body::Enum(_) => panic!("Default implementation for `enum` not yet stabilized"),
    }
    generator.finish()
}

fn write_size_hint<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str(" + D::size_hint(&self.");
    s.push_str(&ident.to_string());
    s.push(')');
}
fn write_ser<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str("D::serialize(self.");
    s.push_str(&ident.to_string());
    s.push_str(",c);");
}
fn write_de<T: std::fmt::Display>(s: &mut String, ident: T) {
    s.push_str(&ident.to_string());
    s.push(':');
    s.push_str(DESERIALIZE);
}
fn write_size(s: &mut String, f: Vec<TokenTree>) {
    let mut ty = proc_macro::TokenStream::new();
    ty.extend(f);
    s.push_str(" + <");
    s.push_str(&ty.to_string());
    s.push_str(" as DataType>::SIZE");
}
fn write_is_dy(s: &mut String, f: Vec<TokenTree>) {
    let mut ty = proc_macro::TokenStream::new();
    ty.extend(f);
    s.push_str("&& <");
    s.push_str(&ty.to_string());
    s.push_str(" as DataType>::IS_DYNAMIC");
}

const DESERIALIZE: &str = "D::deserialize(c)?,";