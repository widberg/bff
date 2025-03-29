use bff_derive::{trivial_class, GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_06_63_02PC, Quat, Vec3f, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LightBodyV1_291_03_06PC {
    #[generic]
    rotation: Quat,
    #[generic]
    direction: Vec3f,
    #[generic]
    color: RGBA,
    #[generic]
    ambient: Vec3f,
    #[generic]
    position: Vec3f,
}

trivial_class!(
    LightV1_291_03_06PC(ObjectLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC),
    LightGeneric
);
