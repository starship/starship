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

fn impl_module_config(dinput: DeriveInput) -> TokenStream {
    let struct_ident = &dinput.ident;
    let (_impl_generics, ty_generics, where_clause) = dinput.generics.split_for_impl();

    if let syn::Data::Struct(data) = dinput.data {
        if let syn::Fields::Named(fields_named) = data.fields {
            let mut tokens = quote! {};

            for field in fields_named.named.iter() {
                let ident = field.ident.as_ref().unwrap();
                let _ty = &field.ty;
                let new_tokens = quote! {
                    if let Some(config_str) = config.get(stringify!(#ident)) {
                        new_module_config.#ident = new_module_config.#ident.load_config(config_str);
                    }
                };
                tokens = quote! {
                    #tokens
                    #new_tokens
                };
            }

            return TokenStream::from(quote! {
                impl<'a> ModuleConfig<'a> for #struct_ident #ty_generics #where_clause {
                    fn load_config(&self, config: &'a toml::Value) -> Self {
                        let mut new_module_config = self.clone();
                        if let toml::Value::Table(config) = config {
                            #tokens
                        }
                        new_module_config
                    }
                }
            });
        }
    }
    TokenStream::from(quote! {})
}
