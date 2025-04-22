use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::helpers::{Quat, RGBA, ResourceLinkHeaderV1_06_63_02PC, Vec3f};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(complete)]
#[br(import(_link_header: &ResourceLinkHeaderV1_06_63_02PC))]
pub struct LightBodyV1_291_03_06PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

trivial_class!(
    LightV1_291_03_06PC(ResourceLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC),
    LightGeneric
);

impl Export for LightV1_291_03_06PC {}
impl Import for LightV1_291_03_06PC {}
