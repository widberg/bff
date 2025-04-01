use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_06_63_02PC, Quat, RGBA, Vec3f};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(complete)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LightBodyV1_291_03_06PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

trivial_class!(
    LightV1_291_03_06PC(ObjectLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC),
    LightGeneric
);
