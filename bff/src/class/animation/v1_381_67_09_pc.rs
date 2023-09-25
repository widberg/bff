use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;
use crate::keyframer::{KeyframerFloatComp, KeyframerVec3fComp, KeyframerMessage, KeyframerRot, KeyframerBezierRot};

#[derive(BinRead, Debug, Serialize)]
struct AnimationNode {
    unknown0: KeyframerRot,
    unknown1: KeyframerBezierRot,
    unknown2: KeyframerVec3fComp,
    unknown3: KeyframerVec3fComp,
    unknown4: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMaterial {
    unknown0: KeyframerFloatComp,
    unknown1: KeyframerFloatComp,
    unknown2: KeyframerVec3fComp,
    unknown3: KeyframerVec3fComp,
    unknown4: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMesh {
    unknown: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMorph {
    unknown: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown12 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown13 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown14 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct AnimationBodyV1_381_67_09PC {
    duration: f32,
    blending: f32,
    c: u16,
    d: u16,
    animation_node: AnimationNode,
    animation_material: AnimationMaterial,
    animation_mesh: AnimationMesh,
    animation_morph: AnimationMorph,
    unknown12s: DynArray<Unknown12>,
    unknown13s: DynArray<Unknown13>,
    unknown14s: DynArray<Unknown14>,
    unknown15s: DynArray<Unknown14>,
}

pub type AnimationV1_381_67_09PC = TrivialClass<LinkHeader, AnimationBodyV1_381_67_09PC>;
