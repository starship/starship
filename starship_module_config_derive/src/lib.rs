extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ModuleConfig)]
pub fn derive_module_config(input: TokenStream) -> TokenStream {
    let dinput = parse_macro_input!(input as DeriveInput);
    impl_module_config(dinput)
}

fn impl_module_config(dinput: DeriveInput) -> proc_macro::TokenStream {
    let struct_ident = &dinput.ident;
    let (_impl_generics, ty_generics, where_clause) = dinput.generics.split_for_impl();

    let mut from_config = quote! {};
    let mut load_config = quote! {};

    if let syn::Data::Struct(data) = dinput.data {
        if let syn::Fields::Named(fields_named) = data.fields {
            let mut load_tokens = quote! {};
            let mut from_tokens = quote! {};

            for field in fields_named.named.iter() {
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                let new_load_tokens = quote! {
                    if let Some(config_str) = config.get(stringify!(#ident)) {
                        new_module_config.#ident = new_module_config.#ident.load_config(config_str);
                    }
                };
                let new_from_tokens = quote! {
                    #ident: <#ty>::from_config(config.get(stringify!(#ident))?)?,
                };

                load_tokens = quote! {
                    #load_tokens
                    #new_load_tokens
                };
                from_tokens = quote! {
                    #from_tokens
                    #new_from_tokens
                }
            }

            load_config = quote! {
                fn load_config(&self, config: &'a toml::Value) -> Self {
                    let mut new_module_config = self.clone();
                    if let toml::Value::Table(config) = config {
                        #load_tokens
                    }
                    new_module_config
                }
            };
            from_config = quote! {
                fn from_config(config: &'a toml::Value) -> Option<Self> {
                    let config = config.as_table()?;

                    Some(#struct_ident {
                        #from_tokens
                    })
                }
            };
        }
    }

    TokenStream::from(quote! {
        impl<'a> ModuleConfig<'a> for #struct_ident #ty_generics #where_clause {
            #from_config
            #load_config
        }
    })
}
