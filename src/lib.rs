use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, NestedMeta, Lit};

#[proc_macro_derive(Constructor, attributes(constructor))]
pub fn constructor_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_constructor_macro(&ast)
}

fn impl_constructor_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let Data::Struct(data_struct) = &ast.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();
                let mut field_initializers = Vec::new();
                let mut constructor_params = Vec::new();

                let mut defaults = std::collections::HashMap::new();

                for attr in &ast.attrs {
                    if attr.path.is_ident("constructor") {
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested_meta in meta_list.nested {
                                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested_meta {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        defaults.insert(nv.path.get_ident().unwrap().to_string(), lit_str.value());
                                    }
                                }
                            }
                        }
                    }
                }

                for field in &fields_named.named {
                    let field_name = &field.ident;
                    let field_type = &field.ty;

                    let default_value = defaults.get(&field_name.as_ref().unwrap().to_string());

                    field_names.push(field_name);
                    field_types.push(field_type);
                    if let Some(value) = default_value {
                        let value_token: proc_macro2::TokenStream = value.parse().expect("Failed to parse default value");
                        field_initializers.push(quote! { #field_name: #value_token });
                    } else {
                        field_initializers.push(quote! { #field_name });
                        constructor_params.push(quote! { #field_name: #field_type });
                    }
                }

                let gen = quote! {
                    impl #name {
                        pub fn new(#(#constructor_params),*) -> Self {
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
