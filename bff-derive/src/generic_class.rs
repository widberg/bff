use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DeriveInput, Ident};

pub fn derive_generic_class(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let gen_name = format!(
        "{}Generic",
        &input
            .ident
            .to_string()
            .split_inclusive("Body")
            .next()
            .unwrap()
    ); // could be better
    let generic_name = Ident::new(&gen_name, gen_name.span());
    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Not a struct"),
    };
    let intos = data
        .fields
        .iter()
        .filter(|f| {
            !f.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("generic"))
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            quote! { #field_ident: object.#field_ident.into() }
        })
        .collect::<Vec<_>>();
    let fill_intos = data
        .fields
        .iter()
        .filter(|f| {
            f.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("generic"))
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            quote! { #field_ident: substitute.#field_ident }
        })
        .collect::<Vec<_>>();
    quote! {
        impl From<#name> for #generic_name {
            fn from(object: #name) -> Self {
                #generic_name {
                    #(#intos),*
                }
            }
        }
        impl crate::traits::TryFromGenericSubstitute<#generic_name, #name> for #name {
            type Error = crate::error::Error;
            fn try_from_generic_substitute(object: #generic_name, substitute: #name) -> crate::BffResult<Self> {
                Ok(#name {
                    #(#intos),*
                    #(#fill_intos),*
                })
            }
        }
    }
}
