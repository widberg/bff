use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Result, Token, Type};

pub struct TrivialClassMacroInput {
    class: Ident,
    link_header: Type,
    body: Type,
    generic: Ident,
}

impl Parse for TrivialClassMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let class = input.parse()?;
        let content;
        parenthesized!(content in input);
        let link_header = content.parse()?;
        let _: Token![,] = content.parse()?;
        let body = content.parse()?;
        let _: Token![,] = input.parse()?;
        let generic = input.parse()?;
        Ok(TrivialClassMacroInput {
            class,
            link_header,
            body,
            generic,
        })
    }
}

pub fn derive_trivial_class(input: TrivialClassMacroInput) -> TokenStream {
    let (class, link_header, body) = (&input.class, &input.link_header, &input.body);
    let from_trivial_to_generic = impl_from_trivial_to_generic(&input);
    let from_generic_to_trivial = impl_from_generic_to_trivial(&input);
    quote! {
        pub type #class = TrivialClass<#link_header, #body>;
        #from_trivial_to_generic
        #from_generic_to_trivial
    }
}

fn impl_from_trivial_to_generic(input: &TrivialClassMacroInput) -> TokenStream {
    let class = &input.class;
    let generic_class = &input.generic;

    let link_header_str = input.link_header.to_token_stream().to_string();
    let link_header = if &link_header_str == "()" {
        quote! {class.body.header.clone().into()}
    } else {
        quote! {class.link_header.into()}
    };

    quote! {
        impl From<#class> for super::generic::#generic_class {
            fn from(
                class: #class,
            ) -> super::generic::#generic_class {
                super::generic::#generic_class {
                    class_name: class.class_name,
                    name: class.name,
                    link_name: class.link_name,
                    compress: class.compress,
                    link_header: #link_header,
                    body: class.body.into(),
                }
            }
        }
    }
}

fn impl_from_generic_to_trivial(input: &TrivialClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;
    let generic_class = &input.generic;

    let link_header_str = input.link_header.to_token_stream().to_string();
    let (body, link_header) = if &link_header_str == "()" {
        (
            quote! {
                {
                    let header = generic.link_header.try_into_specific(substitute.body.header.clone())?;
                    let mut body = generic.body.try_into_specific(substitute.body)?;
                    body.header = header;
                    body
                }
            },
            quote! {()},
        )
    } else {
        (
            quote! {
                generic.body.try_into_specific(substitute.body)?
            },
            quote! {generic.link_header.try_into_specific(substitute.link_header)?},
        )
    };

    quote! {
        use crate::traits::TryIntoSpecific;
        impl crate::traits::TryFromGenericSubstitute<super::generic::#generic_class, #class> for #class {
        type Error = crate::error::Error;

        fn try_from_generic_substitute(
            generic: super::generic::#generic_class,
            substitute: #class,
        ) -> crate::BffResult<#class> {
            let object = #class {
                class_name: generic.class_name,
                name: generic.name,
                link_name: generic.link_name,
                compress: generic.compress,
                link_header: #link_header,
                body: #body,
            };
            Ok(object)
        }
    }}
}
