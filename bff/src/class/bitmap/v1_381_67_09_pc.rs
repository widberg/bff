use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Cursor;

use bff_derive::{GenericClass, ReferencedNames};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite};
use ddsfile::{D3DFormat, Dds, NewD3dParams};
use serde::{Deserialize, Serialize};

use super::generic::BitmapGeneric;
use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::names::Name;
use crate::traits::{Artifact, Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, Copy, Clone)]
#[brw(repr = u16)]
enum BitmapClass {
    Single = 0,
    Cubemap = 2,
}

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, Copy, Clone, Default,
)]
#[brw(repr = u8)]
enum BmFormat {
    #[default]
    BmMultipleBitmaps = 0,
    BmA8l8 = 7,
    BmDxt1 = 14,
    BmDxt5 = 16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, Copy, Clone)]
#[brw(repr = u8)]
enum BitmapClass2 {
    Cubemap2 = 0,
    Single2 = 3,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, Copy, Clone)]
#[brw(repr = u8)]
enum BmTransp {
    NoTransp = 0,
    TranspOne = 1,
    Transp = 2,
    Cubemap = 255,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(name(BitmapHeaderGeneric))]
pub struct LinkHeader {
    #[referenced_names(skip)]
    link_name: Name,
    bitmap_class: BitmapClass,
    #[generic]
    #[serde(skip)]
    width: u32,
    #[generic]
    #[serde(skip)]
    height: u32,
    #[generic]
    #[serde(skip)]
    precalculated_size: u32,
    flags: u8,
    bitmap_type: u8,
    pad: u16,
    layer: f32,
    #[serde(skip)]
    format0: BmFormat,
    #[generic]
    #[serde(skip)]
    mipmap_count: u8,
    four: u8,
    bitmap_class2: BitmapClass2,
    #[serde(skip)]
    format1: BmFormat,
    transparency: BmTransp,
}

#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &LinkHeader))]
pub struct BitmapBodyV1_381_67_09PC {
    #[br(parse_with = until_eof)]
    #[serde(skip)]
    #[generic]
    data: Vec<u8>,
}

pub type BitmapV1_381_67_09PC = TrivialClass<LinkHeader, BitmapBodyV1_381_67_09PC>;

trivial_class_generic!(BitmapV1_381_67_09PC, BitmapGeneric);

impl TryFrom<BmFormat> for D3DFormat {
    type Error = Error;

    fn try_from(value: BmFormat) -> Result<Self, Self::Error> {
        match value {
            BmFormat::BmA8l8 => Ok(Self::A8L8),
            BmFormat::BmDxt1 => Ok(Self::DXT1),
            BmFormat::BmDxt5 => Ok(Self::DXT5),
            _ => Err(Error::UnimplementedImportExport),
        }
    }
}

impl TryFrom<D3DFormat> for BmFormat {
    type Error = Error;

    fn try_from(value: D3DFormat) -> Result<Self, Self::Error> {
        match value {
            D3DFormat::A8L8 => Ok(Self::BmA8l8),
            D3DFormat::DXT1 => Ok(Self::BmDxt1),
            D3DFormat::DXT5 => Ok(Self::BmDxt5),
            _ => Err(Error::UnimplementedImportExport),
        }
    }
}

impl Export for BitmapV1_381_67_09PC {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        let mut dds = Dds::new_d3d(NewD3dParams {
            height: self.link_header.height,
            width: self.link_header.width,
            depth: None,
            format: self.link_header.format0.try_into()?,
            mipmap_levels: Some(self.link_header.mipmap_count as u32),
            caps2: None,
        })
        .unwrap();
        dds.data = self.body.data.clone();
        let mut dds_writer = Cursor::new(Vec::new());
        dds.write(&mut dds_writer).unwrap();
        Ok(HashMap::from([(
            OsString::from("data"),
            Artifact::Dds(dds_writer.into_inner()),
        )]))
    }
}

impl Import for BitmapV1_381_67_09PC {
    fn import(&mut self, artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        let data_name = OsString::from("data");
        let Artifact::Dds(data) = artifacts.get(&data_name).ok_or(Error::ImportBadArtifact)? else {
            return Err(Error::ImportBadArtifact);
        };
        let dds_reader = Cursor::new(data);
        let dds = Dds::read(dds_reader).map_err(|_| Error::ImportBadArtifact)?;
        self.link_header.width = dds.get_width();
        self.link_header.height = dds.get_height();
        self.link_header.precalculated_size = dds.data.len() as u32;
        self.link_header.mipmap_count = dds.get_num_mipmap_levels() as u8;
        self.link_header.format0 = dds.get_d3d_format().unwrap().try_into()?;
        self.link_header.format1 = dds.get_d3d_format().unwrap().try_into()?;
        self.body.data = dds.data;
        Ok(())
    }
}
