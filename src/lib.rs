
extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprPath, Lit, Meta, Type, TypePath};

#[proc_macro_derive(Constructor, attributes(init, new, from))]
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

        let mut default_value: Option<Value> = None;

        for attr in &field.attrs {
            if attr.path().is_ident("init") {
                if let Meta::List(meta_list) = &attr.meta {
                    let inner_token_stream = TokenStream::from(meta_list.tokens.clone());
                    if let Some(lit) = get_lit_from_token_stream(inner_token_stream) {
                        default_value = Some(Value::Lit {lit})
                    }
                    if let Ok(ident) =  syn::parse::<syn::Ident>(TokenStream::from(meta_list.tokens.clone())) {
                        default_value = match ident.to_string().as_str() {
                            "default" => Some(Value::TokenStream {token_stream: quote! { #field_type::default() }.into() } ),
                            _ => panic!("Unknown ident")
                        }
                    }
                }
                
            }
            if attr.path().is_ident("new") { get_initializers(&attr.meta, &mut args, &field_type, &mut default_value, Func::New) }
            if attr.path().is_ident("from") { get_initializers(&attr.meta, &mut args, &field_type, &mut default_value, Func::From) }
        }

        if let Some(value) = default_value  {
            match value {
                Value::Lit { lit }=> {
                    if let Lit::Str(..) = &lit {
                        inits.push(quote! {#field_name: #lit.into()});
                    }
                    else {
                        inits.push(quote! {#field_name: #lit});
                    }
                }
                Value::TokenStream { token_stream } => {
                    inits.push(quote! {#field_name: #token_stream});
                }
            }


        }
        else {
            args.push(quote! {#field_name: #field_type});
            assignments.push(quote! {#field_name})
        }
    }


    let constructor = quote! {
        impl #generics #struct_name #generics {
            pub fn new(#(#args,)*) -> Self {
                Self {
                    #(#assignments,)*
                    #(#inits,)*
                }
            }
        }
    };
    constructor.into()
}

fn get_lit_from_token_stream(token_stream: TokenStream) -> Option<Lit> {
    if let Ok(lit) = syn::parse::<Lit>(token_stream) {
        return Some(lit)
    }
    None
}

fn get_initializers(meta: &Meta, args: &mut Vec<proc_macro2::TokenStream>, field_type: &Type, default_value: &mut Option<Value>, func: Func) {
    if let Meta::List(meta_list) = meta {
        let mut new_args: Vec<proc_macro2::TokenStream> = vec![];
        let inner_token_stream = TokenStream::from(meta_list.tokens.clone());
        if let Some(lit) = get_lit_from_token_stream(inner_token_stream) {
            if let Lit::Str(..) = &lit {
                new_args.push(quote! { #lit.into() }.into());
            }
            else {
                new_args.push(quote! { #lit }.into());
            }

        }
        if let Ok(field) = syn::parse::<syn::FieldValue>(TokenStream::from(meta_list.tokens.clone())) {
            let member = field.member.to_token_stream();

            if let Expr::Path(ExprPath {path, ..}) = field.expr {
                let path_seg = &path.segments[0];
                let ident = &path_seg.ident;
                new_args.push(quote! { #member }.into());
                args.push(quote! {  #member: #ident }.into());
            }

        }
        if let Type::Path(TypePath{path, ..}) = &field_type {
            let path_seg = &path.segments[0];
            let ident = &path_seg.ident;
            match func {
                Func::New =>  *default_value = Some(Value::TokenStream {token_stream: quote! { #ident::new(#(#new_args),*) }.into() }),
                Func::From => *default_value = Some(Value::TokenStream {token_stream: quote! { #ident::from(#(#new_args),*) }.into() }),

            }
        }
    }
}

enum Value {
    Lit{lit: Lit},
    TokenStream{token_stream: proc_macro2::TokenStream}
}

enum Func {
    New,
    From
}