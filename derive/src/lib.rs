use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::*;
use spanned::Spanned;

#[proc_macro_derive(DataType)]
pub fn layout(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let (ser, de) =  match input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let ser = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            bin_layout::DataType::serialize(self.#name, view);
                        }
                    });
                    let de = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            #name: bin_layout::DataType::deserialize(view)?,
                        }
                    });
                    (quote! { #(#ser)*} , quote! { Self { #(#de)* } } )
                }
                Fields::Unnamed(fields) => {
                    let ser = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            bin_layout::DataType::serialize(self.#index, view);
                        }
                    });
                    let de = fields.unnamed.iter().map(|f| {
                        quote_spanned! {f.span()=>
                            bin_layout::DataType::deserialize(view)?,
                        }
                    });
                    (quote! { #(#ser)*} , quote! { Self(#(#de)*) } )
                }
                Fields::Unit => (quote! {} , quote! { Self } )
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    quote! {
        impl #impl_generics  bin_layout::DataType for #name #ty_generics #where_clause {
            fn serialize(self, view: &mut bin_layout::DataView<impl AsMut<[u8]>>) { 
                #ser
            }
            fn deserialize(view: &mut bin_layout::DataView<impl AsRef<[u8]>>) -> bin_layout::Result<Self> { 
                Ok(#de)
            }
        }
    }
    .into()
}
