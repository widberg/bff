pub mod manifest;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_08_40_02_pc;
mod v1_2000_77_18_pc;
mod v1_2002_45_19_pc;
mod v1_22_pc;
mod v2_07_pc;
mod v2_128_92_19_pc;
mod v2_256_38_19_pc;

use std::collections::HashMap;

use bff_derive::bigfiles;
use serde::Serialize;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::BigFileV1_06_63_02PC;
use crate::bigfile::v1_08_40_02_pc::BigFileV1_08_40_02PC;
use crate::bigfile::v1_2000_77_18_pc::BigFileV1_2000_77_18PC;
use crate::bigfile::v1_2002_45_19_pc::BigFileV1_2002_45_19PC;
use crate::bigfile::v1_22_pc::{
    BigFileV1_22PC,
    BigFileV1_22PCNoVersionTriple,
    BigFileV1_22PCNoVersionTripleBlackSheep,
};
use crate::bigfile::v2_07_pc::{BigFileV2_07PCMQFEL, BigFileV2_07PCPROTO, BigFileV2_07PCSHAUN};
use crate::bigfile::v2_128_92_19_pc::BigFileV2_128_92_19PC;
use crate::bigfile::v2_256_38_19_pc::BigFileV2_256_38_19PC;
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
    (AsoboLegacy(1, ..=80), _) => BigFileV1_22PC,
    (AsoboLegacy(1, _) | Asobo(1, 1..=5 | 8, _, _), _) => BigFileV1_08_40_02PC,
    (Asobo(1, 1..=1999, _, _), _) => BigFileV1_06_63_02PC,
    (Asobo(1, 0 | 2000..=2001, _, _), _) => BigFileV1_2000_77_18PC,
    (Asobo(1, 2002.., _, _), _) => BigFileV1_2002_45_19PC,
    (Asobo(2, 128, 92, _), _) => BigFileV2_128_92_19PC,
    (Asobo(2, 256, _, _), _) => BigFileV2_256_38_19PC,
    // (Asobo(2, 128, 52, _), _) => APTR,
}
