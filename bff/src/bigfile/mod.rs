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
use crate::names::Name;

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Resource>,
}

bigfiles! {
    (Kalisto(_, _), _) => v1_22_pc::BigFile,
    (AsoboLegacy(_, _), _) => v1_22_pc::BigFile,
    (Asobo(1, 8, _, _), _) => v1_08_40_02_pc::BigFile,
    (Asobo(1, _, _, _), _) => v1_06_63_02_pc::BigFile,
}
