use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{NumeratorFloat, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Message {
    message_class: u32,
    reciever_name: Name,
    c: u32,
    parameter: f32,
    message_name: Name,
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
struct KeyframerNoFlagsTpl<TKey>
where
    for<'a> TKey: BinRead + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
{
    keyframes: DynArray<TKey>,
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

type Vec3Comp = [NumeratorFloat<i16, 4096>; 3];
type QuatComp = [NumeratorFloat<i16, 2000>; 4];
type KeyMessage = KeyLinearTpl<DynArray<Message>>;
type KeyFloat = KeyTgtTpl<f32>;
type KeyFloatComp = KeyTgtTpl<i16>;
type KeyVec3f = KeyTgtTpl<Vec3f>;
type KeyVec3fComp = KeyTgtTpl<Vec3Comp>;
type KeyRot = KeyLinearTpl<QuatComp>;
type KeyframerMessage = KeyframerNoFlagsTpl<KeyMessage>;
type KeyframerFloat = KeyframerTpl<KeyFloat>;
type KeyframerFloatComp = KeyframerTpl<KeyFloatComp>;
type KeyframerVec3f = KeyframerTpl<KeyVec3f>;
type KeyframerVec3fComp = KeyframerTpl<KeyVec3fComp>;
type KeyframerRot = KeyframerNoFlagsTpl<KeyRot>;

#[derive(BinRead, Debug, Serialize)]
struct RtcAnimationNode {
    unknown_node_name: Name,
    rtc_animation_node_flag: u16,
    unknown0: KeyframerRot,
    unknown1: KeyframerVec3f,
    unknown2: KeyframerVec3f,
    unknown3: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationCamera {
    unknown_node_name: Name,
    animation_camera_flag: u16,
    unknown0: KeyframerFloatComp,
    unknown1: KeyframerFloatComp,
    unknown2: KeyframerFloat,
    unknown3: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationOmni {
    unknown_node_name_name: Name,
    animation_omni_flag: u16,
    unknown0: KeyframerVec3fComp,
    unknown1: KeyframerFloatComp,
    unknown2: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown8 {
    unknown_name_name0: Name,
    unknown_name_name1: Name,
    unknown_name_name2: Name,
    unknown3: u32,
    unknown4: u8,
    unknown_name0: Name,
    unknown_name1: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown9 {
    unknown0: u32,
    unknown_name_name0: Name,
    unknown_name_name1: Name,
    unknown_name_name2: Name,
    unknown_name0: Name,
    unknown_name1: Name,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
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

pub type RtcV1_381_67_09PC = TrivialClass<LinkHeader, RtcBodyV1_381_67_09PC>;
