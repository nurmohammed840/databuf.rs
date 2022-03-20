use virtue::prelude::*;

#[proc_macro_derive(DataType)]
pub fn layout(input: TokenStream) -> TokenStream {
    derive_inner(input).unwrap_or_else(|err| err.into_token_stream())
}

fn derive_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, _attrs, body) = parse.into_generator();
    match body {
        Body::Struct(sbody) => {
            let mut gen = generator.impl_for_with_lifetimes("DataType", ["de"]);
            let (ser, de) = match sbody.fields {
                Fields::Struct(fields) => {
                    let (ser, de) = fields
                        .iter()
                        .map(|(ident, _)| {
                            (format_ser(ident), format!("{}: {},", ident, FORMAT_DE))
                        })
                        .unzip::<_, _, String, String>();
                    (ser, format!("Ok(Self {{{}}})", de))
                }
                Fields::Tuple(fields) => {
                    let (ser, de) = fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| (format_ser(i), FORMAT_DE))
                        .unzip::<_, _, String, Vec<&str>>();

                    (ser, format!("Ok(Self({}))", de.join(",")))
                }
                Fields::Unit => ("".into(), String::from("Ok(Self)")),
                _ => unimplemented!(),
            };

            gen.generate_fn("serialize")
                .with_self_arg(FnSelfArg::TakeSelf)
                .with_arg("cursor", "&mut bin_layout::Cursor<impl AsMut<[u8]>>")
                .with_return_type("bin_layout::Result<()>")
                .body(|fn_body| {
                    fn_body.push_parsed(format!("{} Ok(())", ser))?;
                    Ok(())
                })?;

            gen.generate_fn("deserialize")
                .with_arg("cursor", "&mut bin_layout::Cursor<&'de [u8]>")
                .with_return_type("bin_layout::Result<Self>")
                .body(|fn_body| {
                    fn_body.push_parsed(de)?;
                    Ok(())
                })?;
        }
        Body::Enum(_) => panic!("Enums are not supported, Yet."),
    }
    generator.finish()
}

const FORMAT_DE: &str = "bin_layout::DataType::deserialize(cursor)?";
fn format_ser<T: std::fmt::Display>(ident: T) -> String {
    format!("bin_layout::DataType::serialize(self.{}, cursor)?;", ident)
}
