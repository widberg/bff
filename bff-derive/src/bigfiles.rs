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

    quote! {
        impl BigFile {
            #read_bigfile
            #write_bigfile
        }
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
                    <#body as BigFileIo>::write(self, writer, tag)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, tag: Option<&str>) -> crate::BffResult<()> {
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
