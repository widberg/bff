use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::keyframer::{
    KeyframerFloat,
    KeyframerFloatComp,
    KeyframerMessage,
    KeyframerRot,
    KeyframerVec3f,
    KeyframerVec3fComp,
};
use crate::link_header::ResourceObjectLinkHeader;
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct RtcAnimationNode {
    unknown_node_name: Name,
    rtc_animation_node_flag: u16,
    unknown0: KeyframerRot,
    unknown1: KeyframerVec3f,
    unknown2: KeyframerVec3f,
    unknown3: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct AnimationCamera {
    unknown_node_name: Name,
    animation_camera_flag: u16,
    unknown0: KeyframerFloatComp,
    unknown1: KeyframerFloatComp,
    unknown2: KeyframerFloat,
    unknown3: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct AnimationOmni {
    unknown_node_name_name: Name,
    animation_omni_flag: u16,
    unknown0: KeyframerVec3fComp,
    unknown1: KeyframerFloatComp,
    unknown2: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unknown8 {
    unknown_name_name0: Name,
    unknown_name_name1: Name,
    unknown_name_name2: Name,
    unknown3: u32,
    unknown4: u8,
    unknown_name0: Name,
    unknown_name1: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unknown9 {
    unknown0: u32,
    unknown_name_name0: Name,
    unknown_name_name1: Name,
    unknown_name_name2: Name,
    unknown_name0: Name,
    unknown_name1: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct RtcBodyV1_381_67_09PC {
    duration: f32,
    unknown1s: DynArray<RtcAnimationNode>,
    unknown2s: DynArray<AnimationCamera>,
    unknown_names: DynArray<Name>,
    animation_omnis: DynArray<AnimationOmni>,
    unknown8s: DynArray<Unknown8>,
    unknown9s: DynArray<Unknown9>,
    unknown_names1: DynArray<Name>,
    unknown_names2: DynArray<Name>,
    unknown30: KeyframerMessage,
}

pub type RtcV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, RtcBodyV1_381_67_09PC>;
