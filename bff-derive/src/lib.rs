use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Arm, DeriveInput, Token, LitStr};

#[proc_macro_derive(NamedClass)]
pub fn bff_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let class_name = LitStr::new(format!("{}_Z", name).as_str(), name.span());

    quote! {
        impl crate::traits::NamedClass for #name {
            const NAME: crate::name::Name = crate::crc32::asobo(#class_name.as_bytes());
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn bff_forms(attr: TokenStream, input: TokenStream) -> TokenStream {
    let forms: Vec<Arm> =
        parse_macro_input!(attr with Punctuated::<Arm, Token!(,)>::parse_terminated)
            .into_iter()
            .collect();
    let input = parse_macro_input!(input as DeriveInput);

    let from_object_to_shadow_class = impl_from_object_to_shadow_class(&input, forms);

    quote! {
        #input
        #from_object_to_shadow_class
    }
    .into()
}

fn impl_from_object_to_shadow_class(
    ast: &syn::DeriveInput,
    forms: Vec<Arm>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let arms = forms.iter().map(|form| {
        let attrs = &form.attrs;
        let pat = &form.pat;
        let guard = match &form.guard {
            Some((_, guard)) => quote! { #guard },
            None => quote! {},
        };
        let body = &form.body;
        quote! {
            #(#attrs)*
            #pat #guard => {
                let shadow_class: #body = <#body as crate::traits::TryFromVersionPlatform<&crate::object::Object>>::try_from_version_platform(object, version, platform)?;
                Ok(shadow_class.into())
            }
        }
    }).collect::<Vec<_>>();

    let gen = quote! {
        impl crate::traits::TryFromVersionPlatform<&crate::object::Object> for #name {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                object: &crate::object::Object,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<#name> {
                use crate::versions::Version::*;
                use crate::platforms::Platform::*;
                match (version, platform) {
                    #(#arms)*
                    _ => Err(
                        crate::error::UnimplementedClassError::new(object.name(), <Self as crate::traits::NamedClass>::NAME, version, platform).into(),
                    ),
                }
            }
        }
    };

    gen.into()
}
