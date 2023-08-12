use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{NumeratorFloat, Vec3f};
use crate::name::Name;

type KeyframerRot = KeyframerNoFlagsTpl<KeyRot>;
type KeyframerBezierRot = KeyframerNoFlagsTpl<KeyBezierRot>;
type KeyRot = KeyLinearTpl<QuatComp>;
type KeyBezierRot = KeyTgtTpl<Vec3f>;
type QuatComp = [NumeratorFloat<i16, 2000>; 4];
type Vec3Comp = [NumeratorFloat<i16, 4096>; 3];
type KeyVec3fComp = KeyTgtTpl<Vec3Comp>;
type KeyframerVec3fComp = KeyframerTpl<KeyVec3fComp>;
type KeyframerMessage = KeyframerNoFlagsTpl<KeyMessage>;
type KeyMessage = KeyLinearTpl<DynArray<Message>>;
type KeyframerFloatComp = KeyframerTpl<KeyFloatComp>;
type KeyFloatComp = KeyTgtTpl<i16>;

#[derive(BinRead, Debug, Serialize)]
struct Message {
    message_class: u32,
    reciever_name: Name,
    c: u32,
    parameter: f32,
    message_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum KeyframerInterpolationType {
    FlKeyframerSmooth = 0x01,
    FlKeyframerLinear = 0x02,
    FlKeyframerSquare = 0x03,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerTpl<TKey>
where
    for<'a> TKey: BinRead + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
{
    interpolation_type: KeyframerInterpolationType,
    keyframes: DynArray<TKey>,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerNoFlagsTpl<TKey>
where
    for<'a> TKey: BinRead + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
{
    keyframes: DynArray<TKey>,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyTgtTpl<T>
where
    for<'a> T: BinRead + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    time: f32,
    value: T,
    tangent_in: T,
    #[br(align_after = 4)]
    tangent_out: T,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyLinearTpl<T>
where
    for<'a> T: BinRead + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    time: f32,
    #[br(align_after = 4)]
    value: T,
}

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
pub struct ResourceObject {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
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

pub type AnimationV1_381_67_09PC = TrivialClass<ResourceObject, AnimationBodyV1_381_67_09PC>;
