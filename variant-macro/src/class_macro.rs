use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::model::{TwClassContainer, TwClassField};

pub fn class_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwClassContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_ident = &container.ident;
    let builder_ident = format_ident!("{struct_ident}Builder");

    let fields = container
        .data
        .take_struct()
        .expect("Expected struct fields");

    let base_class = container.class.unwrap_or_default();

    let merger = {
        if let Some(merger) = container.merger {
            let ident = merger.as_ident();
            quote! {#ident}
        } else {
            quote! {tw_merge::DefaultTailwindClassMerge}
        }
    };

    let field_idents = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone());

    let builder_fields = fields.iter().map(|field| {
        let TwClassField { ident, ty, .. } = field;
        quote! { #ident: Option<#ty> }
    });

    let builder_set_methods = fields.iter().map(|field| {
        let TwClassField { ident, ty, .. } = field;
        quote! {
            pub fn #ident(mut self, value: #ty) -> Self {
                self.#ident = Some(value);
                self
            }
        }
    });

    let builder_to_tailwind = {
        let optional_builder_fields = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.as_ref().unwrap_or(&Default::default()).to_class()
            }
        });

        quote! {
            impl tw_utils::ToTailwindClass for #builder_ident{
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    use tw_merge::{ MaybeToTailwindClass, TailwindClassMerger };
                    let class = tw_merge::tw_join!(#base_class, #(#optional_builder_fields),*, class.as_ref());
                    #merger.merge_classes(class)
                }
            }
        }
    };

    let struct_to_tailwind = {
        let field_class_calls = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.to_class()
            }
        });

        quote! {
            impl tw_utils::ToTailwindClass for #struct_ident {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    use tw_merge::{ MaybeToTailwindClass, TailwindClassMerger };
                    let class = tw_merge::tw_join!(#base_class, #(#field_class_calls),*, class.as_ref());
                    #merger.merge_classes(class)
                }
            }
        }
    };

    let gen = quote! {
        impl #struct_ident {
            pub fn variant() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: None,)*
                }
            }
        }

        pub struct #builder_ident {
            #(#builder_fields,)*
        }

        impl #builder_ident {
            #(#builder_set_methods)*

        }
        #builder_to_tailwind

        #struct_to_tailwind
    };

    gen.into()
}