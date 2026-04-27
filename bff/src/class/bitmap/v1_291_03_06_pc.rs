use std::collections::HashMap;
use std::ffi::OsString;

use bff_derive::ReferencedNames;
use binrw::helpers::until_eof;
use binrw::{BinWrite, binread};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::ResourceObjectLinkHeaderV1_06_63_02PC;
use crate::traits::{Artifact, Export, Import};

#[derive(..BffStruct, Clone)]
pub struct BitmapHeader {
    width: u32,
    height: u32,
    precalculated_size: u32,
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
}

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames, JsonSchema)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct BitmapBodyV1_291_03_06PC {
    header: BitmapHeader,
    #[br(parse_with = until_eof)]
    #[serde(skip)]
    data: Vec<u8>,
}

pub type BitmapV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, BitmapBodyV1_291_03_06PC>;

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
        self.body.data.clone_from(data);
        Ok(())
    }
}
