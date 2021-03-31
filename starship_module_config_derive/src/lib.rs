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

            for field in fields_named.named.iter() {
                let ident = field.ident.as_ref().unwrap();

                let new_load_tokens = quote! {
                    stringify!(#ident) => self.#ident.load_config(v),
                };

                load_tokens = quote! {
                    #load_tokens
                    #new_load_tokens
                };
            }

            load_config = quote! {
                fn load_config(&mut self, config: &'a toml::Value) {
                    if let toml::Value::Table(config) = config {
                        config.iter().for_each(|(k, v)| {
                            match k.as_str() {
                                #load_tokens
                                unknown => {
                                    ::log::warn!("Unknown config key '{}'", unknown);
                                },
                            }
                        });
                    }
                }
            };
            from_config = quote! {
                fn from_config(config: &'a toml::Value) -> Option<Self> {
                    let mut out = Self::default();
                    out.load_config(config);
                    Some(out)
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
