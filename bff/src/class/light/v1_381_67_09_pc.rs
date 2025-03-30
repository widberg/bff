use bff_derive::{trivial_class, GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_381_67_09PC, Quat, Vec3f, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(complete)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LightBodyV1_381_67_09PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

trivial_class!(
    LightV1_381_67_09PC(ObjectLinkHeaderV1_381_67_09PC, LightBodyV1_381_67_09PC),
    LightGeneric
);
