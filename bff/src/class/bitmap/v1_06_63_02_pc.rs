use std::collections::HashMap;
use std::ffi::OsString;

use bff_derive::{GenericClass, ReferencedNames};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::{BitmapBodyGeneric, BitmapHeaderGeneric};
use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::ResourceObjectLinkHeaderV1_06_63_02PC;
use crate::traits::{Artifact, Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct BitmapBodyV1_06_63_02PC {
    width: u32,
    height: u32,
    precalculated_size: u32,
    format: u8,
    format_copy: u8,
    palette_format: u8,
    transp_format: u8,
    mipmap_count: u8,
    four: u8,
    flag: u16,
    #[br(parse_with = until_eof)]
    #[serde(skip_serializing)]
    #[generic]
    data: Vec<u8>,
}

pub type BitmapV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, BitmapBodyV1_06_63_02PC>;

impl From<BitmapV1_06_63_02PC> for TrivialClass<Option<BitmapHeaderGeneric>, BitmapBodyGeneric> {
    fn from(value: BitmapV1_06_63_02PC) -> Self {
        let link_header = Some(BitmapHeaderGeneric {
            width: value.body.width,
            height: value.body.height,
            precalculated_size: value.body.precalculated_size,
            mipmap_count: value.body.mipmap_count,
        });

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

impl Export for BitmapV1_06_63_02PC {
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

impl Import for BitmapV1_06_63_02PC {
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
