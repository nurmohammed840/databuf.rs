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
                    let mut _size = String::with_capacity(fields.len() * 30);
                    let mut ser = String::with_capacity(fields.len() * 55);
                    let mut de = String::with_capacity(fields.len() * 55);

                    for (ident, ty) in fields.iter() {
                        _size.push_str(&format!(" + <{} as DataType>::SIZE", ty.type_string()));
                        ser.push_str(&format_ser(ident));
                        de.push_str(&format!("{}: {},", ident, FORMAT_DE));
                    }

                    // log(&_size);

                    (ser, format!("Ok(Self {{ {de} }})"))
                }
                Fields::Tuple(fields) => {
                    let mut _size = String::with_capacity(fields.len() * 35);
                    let mut ser = String::with_capacity(fields.len() * 50);
                    let mut de = Vec::with_capacity(fields.len());

                    _size.push('0');

                    for (i, ty) in fields.iter().enumerate() {
                        _size.push_str(&format!(" + <{} as DataType>::SIZE", ty.type_string()));
                        ser.push_str(&format_ser(i));
                        de.push(FORMAT_DE);
                    }

                    (ser, format!("Ok(Self({}))", de.join(",")))
                }
                Fields::Unit => ("".into(), String::from("Ok(Self)")),
                _ => unimplemented!(),
            };

            // gen.generate_const("SIZE", "usize").with_value(|b| {
            //     b.push_parsed(item)?;
            //     Ok(())
            // });

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
        Body::Enum(_) => panic!("`enum` are not yet supported"),
    }
    generator.finish()
}

const FORMAT_DE: &str = "bin_layout::DataType::deserialize(c)?";

fn format_ser<T: std::fmt::Display>(ident: T) -> String {
    format!("bin_layout::DataType::serialize(self.{}, c);", ident)
}

// fn log<T: std::fmt::Debug>(v: &T) {
//     // std::fs::write("log.txt", v.to_string()).unwrap();
//     std::fs::write("log", format!("{:#?}", v)).unwrap();
// }
