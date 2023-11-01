pub mod manifest;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_08_40_02_pc;
mod v1_22_pc;
mod v2_07_pc;

use std::collections::HashMap;

use bff_derive::bigfiles;
use serde::Serialize;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::BigFileV1_06_63_02PC;
use crate::bigfile::v1_08_40_02_pc::BigFileV1_08_40_02PC;
use crate::bigfile::v1_22_pc::{
    BigFileV1_22PC,
    BigFileV1_22PCNoVersionTriple,
    BigFileV1_22PCNoVersionTripleBlackSheep,
};
use crate::bigfile::v2_07_pc::{BigFileV2_07PCMQFEL, BigFileV2_07PCPROTO, BigFileV2_07PCSHAUN};
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
    (Kalisto(1, 75) | BlackSheep(1, _), _) => BigFileV1_22PCNoVersionTripleBlackSheep,
    (Kalisto(1, _), _) => BigFileV1_22PCNoVersionTriple,
    (BlackSheep(2, ..=7), _) => BigFileV2_07PCPROTO,
    (BlackSheep(2, _), _) => BigFileV2_07PCSHAUN,
    (Ubisoft { .. }, _) => BigFileV2_07PCMQFEL,
    (AsoboLegacy(1, 81..) | Asobo(1, ..=5, _, _) | Asobo(1, 8, _, _), _) => BigFileV1_08_40_02PC,
    (AsoboLegacy(1, _), _) => BigFileV1_22PC,
    (Asobo(1, _, _, _), _) => BigFileV1_06_63_02PC,
}
