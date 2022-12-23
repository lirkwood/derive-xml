use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Lit, Meta, MetaList, NestedMeta};

#[proc_macro_derive(XMLObject)]
pub fn xml_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_xml_object(&ast);
}

#[derive(Debug)]
enum XMLField {
    Attribute {
        default: Option<String>,
    },
    Element {
        default: Option<String>,
        empty: bool,
    },
}

/// Parses xml field directives from a normal structs named field.
fn named_field_type(field: &Field) -> Option<XMLField> {
    for attr in &field.attrs {
        match attr.parse_args::<MetaList>() {
            Ok(list) => {
                if list.path.is_ident("xml_attr") {
                    for token in list.nested {
                        match token {
                            NestedMeta::Meta(Meta::NameValue(directive)) => {
                                if directive.path.is_ident("default") {
                                    if let Lit::Str(default) = directive.lit {
                                        return Some(XMLField::Attribute {
                                            default: Some(default.value()),
                                        });
                                    } else {
                                        panic!("Invalid default for xml attribute (must be literal string)")
                                    }
                                } else {
                                    panic!(
                                        "Invalid directive for xml attribute (must be \"default\")"
                                    )
                                }
                            }
                            _ => panic!(
                                "Invalid directive for xml attribute (must be name/value pair)"
                            ),
                        }
                    }
                    return Some(XMLField::Attribute { default: None });
                } else if list.path.is_ident("xml_elem") {
                    for token in list.nested {
                        match token {
                            NestedMeta::Meta(Meta::NameValue(directive)) => {
                                let mut default = None;
                                let mut empty = true;
                                if directive.path.is_ident("default") {
                                    if let Lit::Str(_default) = directive.lit {
                                        default = Some(_default.value());
                                    } else {
                                        panic!("Invalid default for xml element (must be literal string)")
                                    }
                                } else if directive.path.is_ident("empty") {
                                    if let Lit::Bool(_empty) = directive.lit {
                                        empty = _empty.value;
                                    } else {
                                        panic!("Invalid empty directive for xml element (must be boolean)")
                                    }
                                } else {
                                    panic!("Invalid directive for xml element (must be \"default\" or \"empty\")")
                                }
                                return Some(XMLField::Element { default, empty });
                            }
                            _ => panic!("Invalid xml"),
                        }
                    }
                }
            }
            Err(_) => continue,
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
                panic!("Cannot derive XMLObject for tuple struct.");
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
            panic!("{xml_fields:?}")
        }
    }
    let gen = quote! {};
    return gen.into();
}
