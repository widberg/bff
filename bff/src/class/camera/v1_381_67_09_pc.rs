use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct CameraBodyV1_381_67_09PC {
    angle_of_view: f32,
    zero: f32,
    node_name: Name,
}

pub type CameraV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, CameraBodyV1_381_67_09PC>;
