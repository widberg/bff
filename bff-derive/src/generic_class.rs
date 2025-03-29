use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parenthesized, DataStruct, DeriveInput, Ident};

struct SpecificClass<'a> {
    name: &'a Ident,
    generic_name: Ident,
    data: &'a DataStruct,
    // is_link_header: LitBool,
}

fn simple_parse(input: &DeriveInput) -> SpecificClass {
    let attrs = &input.attrs;
    let name = &input.ident;
    let mut custom_name: Option<Ident> = None;
    // let mut is_link_header: LitBool = LitBool::new(false, false.span());
    for attr in attrs {
        if attr.path().is_ident("generic") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let name: Ident = content.parse()?;
                    custom_name = Some(name);
                    return Ok(());
                }
                // if meta.path.is_ident("link_header") {
                //     let content;
                //     parenthesized!(content in meta.input);
                //     is_link_header = content.parse()?;
                //     return Ok(());
                // }
                Err(meta.error(format!("unknown attribute {:?}", meta.path)))
            })
            .unwrap();
            break;
        }
    }
    let generic_name = match custom_name {
        Some(name) => name,
        None => {
            let gen_name = format!(
                "{}Generic",
                &input
                    .ident
                    .to_string()
                    .split_inclusive("Body")
                    .next()
                    .unwrap()
            );
            Ident::new(&gen_name, gen_name.span())
        }
    };
    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Not a struct"),
    };
    SpecificClass {
        name,
        generic_name,
        data,
        // is_link_header,
    }
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
    let class = simple_parse(input);
    let (name, generic_name) = (class.name, class.generic_name);
    let generic_intos = class
        .data
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
    // let is_link_header = class.is_link_header;
    quote! {
        // impl crate::traits::IsLinkHeader for #name {
        //     fn is_link_header(&self) -> bool {
        //         #is_link_header
        //     }
        // }
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
    let class = simple_parse(input);
    let (name, generic_name) = (class.name, class.generic_name);
    let substitute_intos = class.data.fields.iter().filter(|f| {
            !f.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("generic"))
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            let field_type = &f.ty;
            let mut primitive = true;
            for attr in &f.attrs {
                if attr.path().is_ident("generic") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("non_primitive") {
                            primitive = false;
                        }
                        Ok(())
                    });
                }
            }
            if primitive {
                quote! { #field_ident: generic.#field_ident.into() }
            } else {
                quote! { #field_ident: #field_type::try_from_generic_substitute(generic.#field_ident, substitute.#field_ident)? }
            }
        })
        .collect::<Vec<_>>();
    let fill_intos = class
        .data
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
            fn try_from_generic_substitute(generic: super::generic::#generic_name, substitute: #name) -> crate::BffResult<Self> {
                Ok(#name {
                    #(#substitute_intos),*,
                    #(#fill_intos),*
                })
            }
        }
    }
}
