use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{RGBA, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ObjectDatas {
    unknown_float: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshDataBodyV1_291_03_06PC {
    object_datas: ObjectDatas,
}

pub type MeshDataV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, MeshDataBodyV1_291_03_06PC>;

impl Export for MeshDataV1_291_03_06PC {}
impl Import for MeshDataV1_291_03_06PC {}
