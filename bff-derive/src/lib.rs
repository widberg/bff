use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input, Arm, DeriveInput, Ident, LitStr, Result};

#[proc_macro_derive(NamedClass)]
pub fn bff_named_class(input: TokenStream) -> TokenStream {
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

struct BffClassMacroInput {
    class: Ident,
    forms: Vec<Arm>,
}

impl Parse for BffClassMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let class = input.parse()?;
        let content;
        braced!(content in input);
        let mut forms = Vec::new();
        while !content.is_empty() {
            forms.push(content.parse()?);
        }
        Ok(BffClassMacroInput { class, forms })
    }
}

#[proc_macro]
pub fn bff_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BffClassMacroInput);

    let enum_class = impl_enum_class(&input);

    let from_object_to_shadow_class = impl_from_object_to_shadow_class(&input);

    quote! {
        #enum_class
        #from_object_to_shadow_class
    }
    .into()
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
        #[derive(Serialize, Debug, NamedClass, derive_more::From, derive_more::IsVariant)]
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
                let shadow_class: #body = <#body as crate::traits::TryFromVersionPlatform<&crate::object::Object>>::try_from_version_platform(object, version, platform)?;
                Ok(shadow_class.into())
            }
        }
    }).collect::<Vec<_>>();

    quote! {
        impl crate::traits::TryFromVersionPlatform<&crate::object::Object> for #class {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                object: &crate::object::Object,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<#class> {
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
    }
}

// bilge serialization is incorrect
#[proc_macro_attribute]
pub fn serialize_bits(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = LitStr::new(&name.to_string(), name.span());

    let data = if let syn::Data::Struct(data) = &input.data {
        data
    } else {
        unimplemented!();
    };

    let field_count = data.fields.len();

    let fields = data.fields.iter().filter(|f| {
        if let Some(name) = &f.ident {
            name != "reserved" && name != "_reserved" && name != "padding" && name != "_padding"
        } else {
            true
        }
    });

    let serialize_fields = fields.map(|f| {
        let name = &f.ident;
        let name_str = if let Some(n) = name {
            LitStr::new(&n.to_string(), n.span())
        } else {
            unimplemented!()
        };
        quote! {
            state.serialize_field(#name_str, &self.#name().value())?;
        }
    });

    quote! {
        #input
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                use serde::Serialize;
                let mut state = serializer.serialize_struct(#name_str, #field_count)?;
                #(#serialize_fields)*
                state.end()
            }
        }
    }
    .into()
}
