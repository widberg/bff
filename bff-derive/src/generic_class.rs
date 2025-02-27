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
    let fields = match input.data {
        syn::Data::Struct(data) => data.fields.into_iter(),
        _ => panic!("Not a struct"),
    };
    let intos = fields
        .filter(|f| {
            !f.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("generic"))
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.unwrap(); // for named structs only
            quote! { #field_ident: class.#field_ident.into() }
        })
        .collect::<Vec<_>>();
    quote! {
        impl From<#name> for #generic_name {
            fn from(class: #name) -> #generic_name {
                #generic_name {
                    #(#intos),*
                }
            }
        }
    }
}
