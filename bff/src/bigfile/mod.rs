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
use crate::bigfile::v1_22_pc::{BigFileV1_22PC, BigFileV1_22PCNoVersionTriple, BigFileV1_22PCNoVersionTripleBlackSheep};
use crate::names::Name;

pub static DEFAULT_TAG: &str = "made with <3 by bff contributors (https://github.com/widberg/bff)";

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Resource>,
}

bigfiles! {
    (Kalisto(_, _), _) => BigFileV1_22PCNoVersionTriple,
    (BlackSheep(_, _), _) => BigFileV1_22PCNoVersionTripleBlackSheep,
    (AsoboLegacy(1, 81..) | Asobo(1, ..=5, _, _) | Asobo(1, 8, _, _), _) => BigFileV1_08_40_02PC,
    (AsoboLegacy(_, _), _) => BigFileV1_22PC,
    (Asobo(1, _, _, _), _) => BigFileV1_06_63_02PC,
}
