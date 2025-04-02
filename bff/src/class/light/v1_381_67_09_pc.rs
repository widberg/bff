use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::helpers::{ObjectLinkHeaderV1_381_67_09PC, Quat, RGBA, Vec3f};
use crate::traits::{Export, Import};

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

impl Export for LightV1_381_67_09PC {}
impl Import for LightV1_381_67_09PC {}
