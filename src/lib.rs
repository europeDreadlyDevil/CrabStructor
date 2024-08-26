use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, NestedMeta, Lit, Type, Lifetime, TypePath, Path, TypeReference};
use syn::parse::Parser;

#[proc_macro_derive(Constructor, attributes(init))]
pub fn constructor_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_constructor_macro(&ast)
}

fn impl_constructor_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    if let Data::Struct(data_struct) = &ast.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut field_names = Vec::new();
                let mut field_initializers = Vec::new();
                let mut arg_fields = Vec::new();
                let mut arg_types = Vec::new();

                for field in &fields_named.named {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    println!("{:?}", field_type);
                    arg_fields.push(field_name);
                    arg_types.push(field_type);

                    let mut default_value = None;
                    for attr in &field.attrs {
                        if attr.path.is_ident("init") {
                            let meta = attr.parse_meta().expect("Failed to parse meta");
                            if let Meta::List(meta_list) = meta {
                                for nested_meta in meta_list.nested {
                                    match field_type {
                                        Type::Path(TypePath{path, ..}) => {
                                            default_value = get_value_in_path(path, &nested_meta);
                                        }
                                        Type::Reference(TypeReference{elem, ..}) => match *elem.clone() {
                                            Type::Path(TypePath{path, ..}) => {
                                                default_value = get_value_in_path(&path, &nested_meta);
                                            }
                                            _ => {}
                                        }
                                        _ => {}
                                    }
                                    arg_fields.pop();
                                    arg_types.pop();
                                }
                            }
                        }
                    }

                    field_names.push(field_name);
                    println!("{default_value:?}");
                    if let Some(value) = default_value {
                        match value {
                            Value::I32(num) => {
                                let value_token: proc_macro2::TokenStream = num.parse().expect("Failed to parse default value");
                                field_initializers.push(quote! { #field_name: #value_token });
                            }
                            Value::String(string) => {
                                let value_token: proc_macro2::TokenStream = format!(r#"String::from("{}")"#, string).parse().expect("Failed to parse default value");
                                field_initializers.push(quote! { #field_name: #value_token });
                            }
                            Value::StrRef(str_ref) => {
                                let value_token: proc_macro2::TokenStream = format!(r#""{}""#, str_ref).parse().expect("Failed to parse default value");
                                field_initializers.push(quote! { #field_name: #value_token });
                            }
                            _ => {}
                        }


                    } else {
                        field_initializers.push(quote! { #field_name });
                    }
                }

                let gen = quote! {
                    impl #generics #name #generics {
                        pub fn new(#(#arg_fields: #arg_types),*) -> Self {
                            Self {
                                #(#field_initializers),*
                            }
                        }
                    }
                };
                return gen.into();
            }
            _ => panic!("This macro only works for structs with named fields."),
        }
    }

    panic!("This macro can only be used with structs.");
}

fn get_value_in_path(path: &Path, nested_meta: &NestedMeta) -> Option<Value<String>> {
    if let Path{segments, .. } = path {
        if let Some(path_segment) = segments.last() {
             match path_segment.ident.to_string().as_str() {
                "i32" => if let NestedMeta::Lit(Lit::Int(lit_int)) = &nested_meta {
                    return Some(Value::I32(lit_int.base10_parse::<i32>().expect("Failed to parse int").to_string()))
                }
                "String" => if let NestedMeta::Lit(Lit::Str(lit_str)) = &nested_meta {
                    return Some(Value::String(lit_str.value()))
                }
                "str" => if let NestedMeta::Lit(Lit::Str(lit_str)) = &nested_meta {
                    return Some(Value::StrRef(lit_str.value()))
                }
                _ => return None
            }
        }
        return None
    }
    None
}

#[derive(Debug)]
enum Value<T> {
    I32(T),
    String(T),
    StrRef(T)
}