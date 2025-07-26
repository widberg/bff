use std::collections::HashMap;
use std::ffi::OsString;

use bff_derive::{GenericClass, ReferencedNames};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite, binread};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::generic::{BitmapBodyGeneric, BitmapGeneric, BitmapHeaderGeneric};
use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::ResourceObjectLinkHeaderV1_06_63_02PC;
use crate::traits::{Artifact, Export, Import};

#[derive(
    BinRead,
    Debug,
    Serialize,
    BinWrite,
    Deserialize,
    ReferencedNames,
    GenericClass,
    Clone,
    JsonSchema,
)]
#[generic(name(BitmapHeaderGeneric))]
pub struct BitmapHeader {
    #[generic]
    width: u32,
    #[generic]
    height: u32,
    #[generic]
    precalculated_size: u32,
    flag: u16,
    format: u8,
    #[generic]
    mipmap_count: u8,
    unknown: u8,
}

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass, JsonSchema)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct BitmapBodyV1_291_03_06PC {
    header: BitmapHeader,
    #[br(parse_with = until_eof)]
    #[serde(skip)]
    #[generic]
    data: Vec<u8>,
}

pub type BitmapV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, BitmapBodyV1_291_03_06PC>;

impl From<BitmapV1_291_03_06PC> for BitmapGeneric {
    fn from(value: BitmapV1_291_03_06PC) -> Self {
        let link_header = BitmapHeaderGeneric {
            width: value.body.header.width,
            height: value.body.header.height,
            precalculated_size: value.body.header.precalculated_size,
            mipmap_count: value.body.header.mipmap_count,
        };

        let body = BitmapBodyGeneric {
            data: value.body.data,
        };

        Self {
            class_name: value.class_name,
            name: value.name,
            link_name: value.link_name,
            link_header,
            body,
        }
    }
}

impl Export for BitmapV1_291_03_06PC {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        let data_name = OsString::from("data");
        // TODO: Check the header format field and do something smart with it
        let magic = &self.body.data[..4];
        match magic {
            &[0x44, 0x44, 0x53, 0x20] => Ok(HashMap::from([(
                data_name,
                Artifact::Dds(self.body.data.clone()),
            )])),
            _ => Ok(HashMap::from([(
                data_name,
                Artifact::Binary(self.body.data.clone()),
            )])),
        }
    }
}

impl Import for BitmapV1_291_03_06PC {
    fn import(&mut self, artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        let data_name = OsString::from("data");
        let (Artifact::Dds(data) | Artifact::Binary(data)) =
            artifacts.get(&data_name).ok_or(Error::ImportBadArtifact)?
        else {
            return Err(Error::ImportBadArtifact);
        };
        self.body.data = data.clone();
        Ok(())
    }
}
