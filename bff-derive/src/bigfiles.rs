use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Arm, Result};

pub struct BffBigFileMacroInput {
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

pub fn derive_bigfiles(input: BffBigFileMacroInput) -> TokenStream {
    let read_bigfile = impl_read_bigfile(&input);
    let write_bigfile = impl_write_bigfile(&input);
    let dump_resource = impl_dump_resource(&input);
    let read_resource = impl_read_resource(&input);
    let version_into_name_type = impl_version_into_name_type(&input);

    quote! {
        impl BigFile {
            #read_bigfile
            #write_bigfile
            #dump_resource
            #read_resource
        }

        #version_into_name_type
    }
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::NAME_TYPE;
                    <#body as BigFileIo>::read(reader, version, platform)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn read_platform<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<Self> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            use binrw::BinRead;
            use crate::traits::BigFileIo;
            let endian: crate::Endian = platform.into();
            let version: crate::bigfile::versions::Version = crate::helpers::FixedStringNull::<256>::read_be(reader)?.as_str().into();
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::NAME_TYPE;
                    <#body as BigFileIo>::write(self, writer, tag)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, tag: Option<&str>) -> crate::BffResult<()> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
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

fn impl_dump_resource(input: &BffBigFileMacroInput) -> proc_macro2::TokenStream {
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::NAME_TYPE;
                    Ok(<#body as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn dump_resource<W: std::io::Write + std::io::Seek>(&self, resource: &crate::bigfile::resource::Resource, writer: &mut W) -> crate::BffResult<()> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            use crate::traits::BigFileIo;
            let platform = self.manifest.platform;
            let endian: crate::Endian = platform.into();
            let version = &self.manifest.version;
            match (version.clone(), platform) {
                #(#arms)*
                (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
            }
        }
    }
}

fn impl_read_resource(input: &BffBigFileMacroInput) -> proc_macro2::TokenStream {
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
                    crate::names::names().lock().unwrap().name_type = <#body as BigFileIo>::NAME_TYPE;
                    Ok(<#body as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn read_resource<R: std::io::Read + std::io::Seek>(&self, reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            use crate::traits::BigFileIo;
            let platform = self.manifest.platform;
            let endian: crate::Endian = platform.into();
            let version = &self.manifest.version;
            match (version.clone(), platform) {
                #(#arms)*
                (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
            }
        }
    }
}

fn impl_version_into_name_type(input: &BffBigFileMacroInput) -> proc_macro2::TokenStream {
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
                #pat #guard => Ok(<#body as BigFileIo>::NAME_TYPE),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl TryFrom<&crate::bigfile::versions::Version> for crate::names::NameType {
            type Error = crate::BffError;

            fn try_from(version: &crate::bigfile::versions::Version) -> Result<crate::names::NameType, Self::Error> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                match (version.clone(), PC) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionError::new(version).into()),
                }
            }
        }
    }
}
