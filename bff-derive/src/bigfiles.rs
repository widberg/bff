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
        Ok(Self { forms })
    }
}

pub fn derive_bigfiles(input: BffBigFileMacroInput) -> TokenStream {
    let read_bigfile = impl_read_bigfile(&input);
    let write_bigfile = impl_write_bigfile(&input);
    let (dump_resource, dump_resource_resource) = impl_dump_resource(&input);
    let (read_resource, read_resource_resource) = impl_read_resource(&input);
    let version_into_name_type = impl_version_into_name_type(&input);

    quote! {
        impl BigFile {
            #read_bigfile
            #write_bigfile
            #dump_resource
            #read_resource
        }

        impl crate::bigfile::resource::Resource {
            #dump_resource_resource
            #read_resource_resource
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
        pub fn read_platform<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::bigfile::platforms::Platform, version_override: &Option<crate::bigfile::versions::Version>) -> crate::BffResult<Self> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            use binrw::BinRead;
            use crate::traits::BigFileIo;
            let endian: crate::Endian = platform.into();
            let version: crate::bigfile::versions::Version = crate::helpers::FixedStringNull::<256>::read_be(reader)?.as_str().into();
            let version = version_override.clone().unwrap_or(version);
            match (&version, platform) {
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
        pub fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform_override: Option<crate::bigfile::platforms::Platform>, version_override: &Option<crate::bigfile::versions::Version>, tag: Option<&str>) -> crate::BffResult<()> {
            use crate::bigfile::versions::Version::*;
            use crate::bigfile::platforms::Platform::*;
            use binrw::BinWrite;
            use crate::traits::BigFileIo;
            let platform = platform_override.unwrap_or(self.manifest.platform);
            let endian: crate::Endian = platform.into();
            let version = &self.manifest.version;
            let version = version_override.as_ref().unwrap_or(version);
            let version_string = version.to_string();
            crate::helpers::FixedStringNull::<256>::write_be(&version_string.into(), writer)?;
            match (version.clone(), platform) {
                #(#arms)*
                (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
            }
        }
    }
}

fn impl_dump_resource(
    input: &BffBigFileMacroInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
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

    (
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

            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(&self, resource: &crate::bigfile::resource::Resource, writer: &mut W) -> crate::BffResult<()> {
                let platform = self.manifest.platform;
                let version = &self.manifest.version;
                crate::bigfile::resource::Resource::dump_bff_resource(resource, writer, platform, version)
            }
        },
        quote! {
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                let resource = self;
                match (version.clone(), platform) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                <crate::bigfile::resource::BffResourceHeader as binrw::BinWrite>::write(&crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version: version.clone(),
                }, writer)?;
                let resource = self;
                match (version.clone(), platform) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }
        },
    )
}

fn impl_read_resource(
    input: &BffBigFileMacroInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
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

    (
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
            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(&self, reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version,
                } = <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }
        },
        quote! {
            pub fn read_resource<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version,
                } = <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    #(#arms)*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }
        },
    )
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
