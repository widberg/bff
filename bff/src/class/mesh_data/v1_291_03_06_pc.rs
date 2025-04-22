use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::RGBA;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ResourceDatas {
    unknown_float: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct MeshDataBodyV1_291_03_06PC {
    resource_datas: ResourceDatas,
}

pub type MeshDataV1_291_03_06PC = TrivialClass<(), MeshDataBodyV1_291_03_06PC>;

impl Export for MeshDataV1_291_03_06PC {}
impl Import for MeshDataV1_291_03_06PC {}
