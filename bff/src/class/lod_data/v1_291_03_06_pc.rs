use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffOption, DynArray, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Extended {
    padding: [u8; 24],
    flags: u32,
    zero0: u32,
    unknown0: u32,
    zero1s: [u32; 3],
    scale: Vec3f,
    zero2s: [u32; 3],
    unknown1: u32,
    zero3s: [u32; 4],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct LodDataBodyV1_291_03_06PC {
    flags: u32,
    mesh_data_names: DynArray<Name>,
    zero: u32,
    extended: BffOption<Extended>,
}

pub type LodDataV1_291_03_06PC = TrivialClass<(), LodDataBodyV1_291_03_06PC>;

impl Export for LodDataV1_291_03_06PC {}
impl Import for LodDataV1_291_03_06PC {}
