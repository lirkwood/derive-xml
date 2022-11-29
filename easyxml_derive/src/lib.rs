use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DataStruct, DeriveInput, Field, Fields, Lit, LitStr, MetaList,
    NestedMeta, Path, Token,
};

#[proc_macro_derive(XMLObject)]
pub fn xml_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_xml_object(&ast);
}

enum XMLField {
    Attribute {
        default: Option<String>,
    },
    Child {
        default: Option<String>,
        empty: bool,
    },
}

fn named_field_type(field: &Field) -> Option<XMLField> {
    for attr in &field.attrs {
        match attr.parse_args::<MetaList>() {
            Ok(list) => {
                if list.path.is_ident("xml") {
                    for token in list.nested {
                        match token {
                            NestedMeta::Lit(lit) => {
                                if let lit.par
                            }
                            NestedMeta::Meta(meta) => {}
                        }
                    }
                }
            }
            Err(err) => continue,
        }
    }

    return None;
}

fn impl_xml_object(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Data::Struct(stct) => return impl_for_struct(stct),
        Data::Enum(_enm) => todo!("Implement enum"),
        Data::Union(_uni) => todo!("Implement union"),
    };
}

fn impl_for_struct(stct: &DataStruct) -> TokenStream {
    match &stct.fields {
        Fields::Unit => {}
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                // if fields.unnamed.first().unwrap().ty
            } else if fields.unnamed.len() > 1 {
                panic!("Cannot derive PSMLObject for tuple struct.");
            }
        }
        Fields::Named(fields) => {
            let mut xml_fields = vec![];
            for field in &fields.named {
                match named_field_type(&field) {
                    None => continue,
                    Some(xmlfield) => xml_fields.push(xmlfield),
                }
            }
        }
    }
    let gen = quote! {};
    return gen.into();
}
