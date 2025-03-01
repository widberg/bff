use bff_derive::{GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_381_67_09PC, Quat, Vec3f, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LightBodyV1_381_67_09PC {
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

pub type LightV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, LightBodyV1_381_67_09PC>;
