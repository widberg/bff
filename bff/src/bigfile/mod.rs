pub mod manifest;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_08_40_02_pc;
mod v1_22_pc;

use std::collections::HashMap;

use bff_derive::bigfiles;
use serde::Serialize;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::BigFileV1_06_63_02PC;
use crate::bigfile::v1_08_40_02_pc::BigFileV1_08_40_02PC;
use crate::bigfile::v1_22_pc::BigFileV1_22PC;
use crate::names::Name;

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Resource>,
}

bigfiles! {
    (Kalisto(_, _), _) => BigFileV1_22PC,
    (AsoboLegacy(_, _), _) => BigFileV1_22PC,
    (Asobo(1, 8, _, _), _) => BigFileV1_08_40_02PC,
    (Asobo(1, _, _, _), _) => BigFileV1_06_63_02PC,
}
