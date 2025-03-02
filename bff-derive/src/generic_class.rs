use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{DataStruct, DeriveInput, Field, Ident};

fn simple_parse(input: &DeriveInput) -> (&Ident, Ident, &DataStruct) {
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
        syn::Data::Struct(ref data) => data,
        _ => panic!("Not a struct"),
    };
    (name, generic_name, &data)
}

pub fn derive_generic_class(input: DeriveInput) -> TokenStream {
    let from_specific_to_generic = impl_from_specific_to_generic(&input);
    let from_generic_substitute = impl_from_generic_substitute(&input);
    quote! {
        #from_specific_to_generic
        #from_generic_substitute
    }
}

fn impl_from_specific_to_generic(input: &DeriveInput) -> TokenStream {
    let (name, generic_name, data) = simple_parse(input);
    let generic_intos = data
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
    quote! {
        impl From<#name> for super::generic::#generic_name {
            fn from(object: #name) -> Self {
                super::generic::#generic_name {
                    #(#generic_intos),*
                }
            }
        }
    }
}

fn impl_from_generic_substitute(input: &DeriveInput) -> TokenStream {
    let (name, generic_name, data) = simple_parse(input);
    let substitute_intos = data.fields.iter().filter(|f| {
            !f.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("generic"))
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            let field_type = f.ty.clone();
            let mut primitive = true;
            for attr in &f.attrs {
                if attr.path().is_ident("generic") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("non_primitive") {
                            primitive = false;
                        }
                        return Ok(());
                    });
                }
            }
            if primitive {
                quote! { #field_ident: object.#field_ident.into() }
            } else {
                quote! { #field_ident: #field_type::try_from_generic_substitute(object.#field_ident, substitute.#field_ident)? }
            }
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
        impl crate::traits::TryFromGenericSubstitute<super::generic::#generic_name, #name> for #name {
            type Error = crate::error::Error;
            fn try_from_generic_substitute(object: super::generic::#generic_name, substitute: #name) -> crate::BffResult<Self> {
                Ok(#name {
                    #(#substitute_intos),*,
                    #(#fill_intos),*
                })
            }
        }
    }
}
