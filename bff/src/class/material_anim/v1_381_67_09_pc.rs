use bilge::prelude::{bitsize, u1, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Vec2f, Vec3f, Vec4f};
use crate::name::Name;

type KeyFlag = KeyLinearTpl<u32>;
type KeyHdl = KeyLinearTpl<Name>;
type KeyFloatLinearComp = KeyLinearTpl<i16>;
type KeyVec2fLinear = KeyLinearTpl<Vec2f>;
type KeyVec3fLinear = KeyLinearTpl<Vec3f>;
type KeyVec4fLinear = KeyLinearTpl<Vec4f>;
type KeyframerFlag = KeyframerNoFlagsTpl<KeyFlag>;
type KeyframerHdl = KeyframerNoFlagsTpl<KeyHdl>;
type KeyframerFloatLinearComp = KeyframerTpl<KeyFloatLinearComp>;
type KeyframerVec2fLinear = KeyframerTpl<KeyVec2fLinear>;
type KeyframerVec3fLinear = KeyframerTpl<KeyVec3fLinear>;
type KeyframerVec4fLinear = KeyframerTpl<KeyVec4fLinear>;

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
    Smooth = 0x01,
    Linear = 0x02,
    Square = 0x03,
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

#[bitsize(8)]
#[derive(BinRead, DebugBits, Serialize)]
struct MaterialAnimFlags {
    fl_mat_play: u1,
    fl_mat_played: u1,
    fl_mat_playonce: u1,
    fl_mat_neveragain: u1,
    fl_mat_autostart: u1,
    flag_5: u1,
    flag_6: u1,
    flag_7: u1,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct MaterialAnimBodyV1_381_67_09PC {
    bitmap_name_keyframer: KeyframerHdl,
    scroll_keyframer: KeyframerVec2fLinear,
    scale_keyframer: KeyframerVec2fLinear,
    rotation_keyframer: KeyframerFloatLinearComp,
    diffuse_keyframer: KeyframerVec3fLinear,
    emission_keyframer: KeyframerVec3fLinear,
    alpha_keyframer: KeyframerFloatLinearComp,
    vec4f_keyframer0: KeyframerVec4fLinear,
    params_keyframer: KeyframerVec4fLinear,
    render_flag_keyframer: KeyframerFlag,
    object_flag_keyframer: KeyframerFlag,
    base_material_name: Name,
    duration: f32,
    flags: MaterialAnimFlags,
}

pub type MaterialAnimV1_381_67_09PC = TrivialClass<LinkHeader, MaterialAnimBodyV1_381_67_09PC>;
