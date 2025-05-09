use std::collections::HashMap;
use std::ffi::OsString;

use bff_derive::{GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::UserDefineGeneric;
use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::{PascalString, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::traits::{Artifact, Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct UserDefineBodyV1_291_03_06PC {
    #[serde(skip)]
    #[generic]
    pub data: PascalString,
}

pub type UserDefineV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, UserDefineBodyV1_291_03_06PC>;

trivial_class_generic!(UserDefineV1_291_03_06PC, UserDefineGeneric);

impl Export for UserDefineV1_291_03_06PC {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        Ok(HashMap::from([(
            OsString::from("data"),
            Artifact::Text(self.body.data.to_string()),
        )]))
    }
}

impl Import for UserDefineV1_291_03_06PC {
    fn import(&mut self, artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        let data_name = OsString::from("data");
        let Artifact::Text(data) = artifacts.get(&data_name).ok_or(Error::ImportBadArtifact)?
        else {
            return Err(Error::ImportBadArtifact);
        };
        self.body.data = PascalString::from(data.clone());
        Ok(())
    }
}
