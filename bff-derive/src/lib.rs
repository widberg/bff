use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, parse_macro_input, Arm, Data, DeriveInput, Fields, Ident, LitStr, Result};

#[proc_macro_derive(NamedClass)]
pub fn bff_named_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let class_name = LitStr::new(format!("{}_Z", name).as_str(), name.span());
    let class_name_legacy = LitStr::new(&name.to_string().to_uppercase(), name.span());

    // This mess can go away once https://github.com/rust-lang/rust/issues/76001 is stabilized
    quote! {
        impl crate::traits::NamedClass<crate::names::NameAsobo32> for #name {
            const NAME: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameAsoboAlternate32> for #name {
            const NAME: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameKalisto32> for #name {
            const NAME: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameAsobo64> for #name {
            const NAME: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<&'static str> for #name {
            const NAME: &'static str = #class_name;
            const NAME_LEGACY: &'static str = #class_name_legacy;
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
    let from_shadow_class_to_object = impl_from_shadow_class_to_object(&input);

    quote! {
        #enum_class
        #from_object_to_shadow_class
        #from_shadow_class_to_object
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

    quote! {
        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for #class {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                object: &crate::bigfile::resource::Resource,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<#class> {
                use crate::versions::Version::*;
                use crate::platforms::Platform::*;
                match (version.clone(), platform) {
                    #(#arms)*
                    _ => Err(
                        // TODO: Pick the right name based on the algorithm and suffix for the current BigFile
                        crate::error::UnimplementedClassError::new(object.name, <Self as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME.into(), version, platform).into(),
                    ),
                }
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

    quote! {
        impl crate::traits::TryFromVersionPlatform<&#class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                class: &#class,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::versions::Version::*;
                use crate::platforms::Platform::*;
                match class {
                    #(#arms)*
                }
            }
        }
    }
}

struct BffBigFileMacroInput {
    forms: Vec<Arm>,
}

impl Parse for BffBigFileMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut forms = Vec::new();
        while !input.is_empty() {
            forms.push(input.parse()?);
        }
        Ok(BffBigFileMacroInput { forms })
    }
}

#[proc_macro]
pub fn bigfiles(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BffBigFileMacroInput);

    let read_bigfile = impl_read_bigfile(&input);
    let write_bigfile = impl_write_bigfile(&input);

    quote! {
        impl BigFile {
            #read_bigfile
            #write_bigfile
        }
    }
    .into()
}

fn impl_read_bigfile(input: &BffBigFileMacroInput) -> proc_macro2::TokenStream {
    let arms = input
        .forms
        .iter()
        .map(|form| {
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::name_type(version.clone(), platform);
                    <#body as BigFileIo>::read(reader, version, platform)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn read_platform<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::platforms::Platform) -> crate::BffResult<Self> {
            use crate::versions::Version::*;
            use crate::platforms::Platform::*;
            use binrw::BinRead;
            use crate::traits::BigFileIo;
            let endian: crate::Endian = platform.into();
            let version: crate::versions::Version = crate::helpers::FixedStringNull::<256>::read_be(reader)?.as_str().into();
            match (version.clone(), platform) {
                #(#arms)*
                _ => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
            }
        }
    }
}

fn impl_write_bigfile(input: &BffBigFileMacroInput) -> proc_macro2::TokenStream {
    let arms = input
        .forms
        .iter()
        .map(|form| {
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::name_type(version.clone(), platform);
                    <#body as BigFileIo>::write(self, writer)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W) -> crate::BffResult<()> {
            use crate::versions::Version::*;
            use crate::platforms::Platform::*;
            use binrw::BinWrite;
            use crate::traits::BigFileIo;
            let platform = self.manifest.platform;
            let endian: crate::Endian = platform.into();
            let version = &self.manifest.version;
            let version_string = version.to_string();
            crate::helpers::FixedStringNull::<256>::write_be(&version_string.into(), writer)?;
            match (version.clone(), platform) {
                #(#arms)*
                (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
            }
        }
    }
}

#[proc_macro_derive(ReferencedNames)]
pub fn referenced_names(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    for param in &mut input.generics.params {
        if let syn::GenericParam::Type(ty) = param {
            ty.bounds
                .push(syn::parse_quote!(crate::traits::ReferencedNames));
        }
    }
    let generics = &mut input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .map(|field| {
                        let name = field.ident.as_ref().unwrap();
                        quote! {
                            names.extend(self.#name.names());
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    #(#fields)*
                }
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! {
                            names.extend(self.#i.names());
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    #(#fields)*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(data) => {
            let variants = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Named(named) => {
                            let (names, fields): (Vec<_>, Vec<_>) = named
                                .named
                                .iter()
                                .map(|field| {
                                    let name = field.ident.as_ref().unwrap();
                                    (
                                        name,
                                        quote! {
                                                names.extend(#name.names());
                                        },
                                    )
                                })
                                .unzip();

                            quote! {
                                #name::#variant_name { #(#names,)* } => {
                                    #(#fields)*
                                }
                            }
                        }
                        Fields::Unnamed(unnamed) => {
                            let (fields, names): (Vec<_>, Vec<_>) = unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, _)| {
                                    let ident =
                                        Ident::new(format!("field_{}", i).as_str(), variant.span());
                                    (
                                        quote! {
                                            names.extend(#ident.names());
                                        },
                                        ident,
                                    )
                                })
                                .unzip();

                            quote! {
                                #name::#variant_name(#(#names,)*) => {
                                    #(#fields)*
                                }
                            }
                        }
                        Fields::Unit => {
                            quote! { #name::#variant_name => {} }
                        }
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                match self {
                    #(#variants)*
                }
            }
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };

    quote! {
        impl #impl_generics crate::traits::ReferencedNames for #name #ty_generics #where_clause {
            fn names(&self) -> std::collections::HashSet<crate::names::Name> {
                let mut names = std::collections::HashSet::new();
                #body
                names
            }
        }
    }
    .into()
}
