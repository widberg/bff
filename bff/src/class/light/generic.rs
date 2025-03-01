use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

// use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Quat, Vec3f, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
// #[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LightBodyGeneric {
    pub rotation: Quat,
    pub direction: Vec3f,
    pub color: RGBA,
    pub ambient: Vec3f,
    pub position: Vec3f,
}

// pub type LightV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC>;
