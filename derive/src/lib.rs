
extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Literal, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, LitStr, MacroDelimiter, Meta};

#[proc_macro_derive(Constructor, attributes(init))]
pub fn constructor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("Error by get struct fields")
    };

    let mut args = vec![];
    let mut inits = vec![];
    let mut assignments = vec![];

    for field in fields.iter() {
        let field_name = field.ident.clone().unwrap();
        let field_type = field.ty.clone();

        let mut default_value: Option<Lit> = None;

        for attr in &field.attrs {
            if attr.path().is_ident("init") {
                if let Meta::List(meta_list) = &attr.meta {
                    println!("{meta_list:?}");
                    let input = syn::parse::<Lit>(TokenStream::from(meta_list.tokens.clone()));
                    println!("{input:?}");
                    if let Ok(lit) = input {
                        if let Lit::Str(lit) = &lit {

                        }
                        default_value = Some(lit)
                    }
                }
            }
        }

        if let Some(value) = default_value  {
            if let Lit::Str(..) = &value {
                inits.push(quote! {#field_name: #value.into()});
            }
            else {
                inits.push(quote! {#field_name: #value});
            }

        }
        else {
            args.push(quote! {#field_name: #field_type});
            assignments.push(quote! {#field_name})
        }
    }


    let constructor = quote! {
        impl #generics #struct_name #generics {
            pub fn new(#(#args),*) -> Self {
                Self {
                    #(#assignments),*
                    #(#inits),*
                }
            }
        }
    };
    constructor.into()
}