use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(XMLObject)]
pub fn xml_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_xml_object(&ast);
}

fn impl_xml_object(ast: &DeriveInput) -> TokenStream {
    // let mut name = None;
    for attr in &ast.attrs {
        if attr.path.is_ident("xml") {
            // for token in &attr.tokens {
            //     match token {
            //         TokenTr::Group(group) => {}
            //         _ => {}
            //     }
            // }
        }
    }

    let gen = quote! {};
    return gen.into();
}
