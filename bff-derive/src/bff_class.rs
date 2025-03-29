use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, Arm, Attribute, Ident, Result};

pub struct BffClassMacroInput {
    class: Ident,
    forms: Vec<Arm>,
    has_generic: bool,
}

impl Parse for BffClassMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_inner)?;
        let has_generic = !attrs
            .iter()
            .filter(|attr| attr.path().is_ident("generic"))
            .collect::<Vec<_>>()
            .is_empty();
        let class = input.parse()?;
        let content;
        braced!(content in input);
        let mut forms = Vec::new();
        while !content.is_empty() {
            forms.push(content.parse()?);
        }
        Ok(BffClassMacroInput {
            class,
            forms,
            has_generic,
        })
    }
}

pub fn derive_bff_class(input: BffClassMacroInput) -> TokenStream {
    let enum_class = impl_enum_class(&input);

    let from_object_to_shadow_class = impl_from_object_to_shadow_class(&input);
    let from_shadow_class_to_object = impl_from_shadow_class_to_object(&input);
    let from_shadow_class_to_generic = impl_from_shadow_class_to_generic(&input);

    if input.has_generic {
        quote! {
            #enum_class
            #from_object_to_shadow_class
            #from_shadow_class_to_object
            #from_shadow_class_to_generic
        }
    } else {
        quote! {
            #enum_class
            #from_object_to_shadow_class
            #from_shadow_class_to_object
        }
    }
}

fn impl_enum_class(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let variants = input
        .forms
        .iter()
        .map(|form| {
            let body = &form.body;
            quote! { #body(#body) }
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, bff_derive::NamedClass, derive_more::From, derive_more::IsVariant, bff_derive::ReferencedNames)]
        pub enum #class {
            #(#variants),*
        }
    }
}

fn impl_from_object_to_shadow_class(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let arms = input.forms.iter().map(|form| {
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
                let shadow_class: #body = <&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<#body>>::try_into_version_platform(object, version, platform)?;
                Ok(shadow_class.into())
            }
        }
    }).collect::<Vec<_>>();

    let body = if arms.is_empty() {
        quote! {
            todo!()
        }
    } else {
        quote! {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            match (version.clone(), platform) {
                #(#arms)*
                _ => Err(
                    // TODO: Pick the right name based on the algorithm and suffix for the current BigFile
                    crate::error::UnimplementedClassError::new(object.name, <Self as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME.into(), version, platform).into(),
                ),
            }
        }
    };

    quote! {
        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for #class {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                object: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<#class> {
                #body
            }
        }
    }
}

fn impl_from_shadow_class_to_object(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let arms = input.forms.iter().map(|form| {
        let attrs = &form.attrs;
        let body = &form.body;
        quote! {
            #(#attrs)*
            #class::#body(class) => {
                let object: crate::bigfile::resource::Resource = <&#body as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class, version, platform)?;
                Ok(object)
            }
        }
    }).collect::<Vec<_>>();

    let body = if arms.is_empty() {
        quote! {
            todo!()
        }
    } else {
        quote! {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            match class {
                #(#arms)*
            }
        }
    };

    quote! {
        impl crate::traits::TryFromVersionPlatform<&#class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                class: &#class,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                #body
            }
        }
    }
}

fn impl_from_shadow_class_to_generic(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;
    let generic_class_str = format!("{}Generic", class);
    let generic_class = Ident::new(&generic_class_str, generic_class_str.span());

    let arms = input
        .forms
        .iter()
        .map(|form| {
            let attrs = &form.attrs;
            let body = &form.body;
            quote! {
                #(#attrs)*
                #class::#body(class) => {
                    class.into()
                }
            }
        })
        .collect::<Vec<_>>();

    let body = if arms.is_empty() {
        quote! {
            todo!()
        }
    } else {
        quote! {
            match class {
                #(#arms)*
            }
        }
    };

    quote! {
        impl From<#class> for generic::#generic_class {
            fn from(
                class: #class,
            ) -> generic::#generic_class {
                #body
            }
        }
    }
}
