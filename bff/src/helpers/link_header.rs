use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite};

use super::{BffBox, DynArray, Sphere};
use crate::names::Name;

#[derive(..BffStruct)]
pub struct ResourceObjectLinkHeaderV1_381_67_09PC {
    #[referenced_names(skip)]
    link_name: Name,
}

#[bitsize(32)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchemaBits,
)]
pub struct ObjectDatasFlagsV1_381_67_09PC {
    hide: u1,
    code_control: u1,
    cloned: u1,
    skinned: u1,
    morphed: u1,
    vreflect: u1,
    hide_shadow: u1,
    static_shadow: u1,
    vp0_hide: u1,
    vp1_hide: u1,
    vp2_hide: u1,
    vp3_hide: u1,
    unknown13: u1,
    unknown14: u1,
    unknown15: u1,
    unknown16: u1,
    unknown17: u1,
    unknown18: u1,
    unknown19: u1,
    unknown20: u1,
    unknown21: u1,
    unknown22: u1,
    unknown23: u1,
    unknown24: u1,
    unknown25: u1,
    unknown26: u1,
    unknown27: u1,
    unknown28: u1,
    unknown29: u1,
    unknown30: u1,
    unknown31: u1,
    unknown32: u1,
}

#[bitsize(32)]
#[derive(
    BinRead,
    FromBits,
    DebugBits,
    SerializeBits,
    BinWrite,
    DeserializeBits,
    ReferencedNames,
    JsonSchemaBits,
)]
pub struct ObjectFlagsV1_381_67_09PC {
    init: u1,
    max_bsphere: u1,
    skinned: u1,
    morphed: u1,
    orientedbbox: u1,
    no_seaddisplay: u1,
    no_seadcollide: u1,
    no_display: u1,
    transparent: u1,
    optimized_vertex: u1,
    linear_mapping: u1,
    skinned_with_one_bone: u1,
    light_baked: u1,
    light_baked_with_material: u1,
    shadow_receiver: u1,
    no_tesselate: u1,
    unknown17: u1,
    unknown18: u1,
    unknown19: u1,
    unknown20: u1,
    unknown21: u1,
    unknown22: u1,
    unknown23: u1,
    unknown24: u1,
    unknown25: u1,
    unknown26: u1,
    unknown27: u1,
    unknown28: u1,
    unknown29: u1,
    unknown30: u1,
    unknown31: u1,
    unknown32: u1,
}

#[derive(..BffStruct)]
#[brw(repr = u16)]
pub enum ObjectType {
    Points = 0,
    Surface = 1,
    Spline = 2,
    Skin = 3,
    RotShape = 4,
    Lod = 5,
    Mesh = 6,
    Camera = 7,
    SplineZone = 9,
    Occluder = 10,
    CameraZone = 11,
    Light = 12,
    HFog = 13,
    CollisionVol = 14,
    Emiter = 15,
    Omni = 16,
    Graph = 17,
    Particles = 18,
    Flare = 19,
    HField = 20,
    Tree = 21,
    GenWorld = 22,
    Road = 23,
    GenWorldSurface = 24,
    SplineGraph = 25,
    WorldRef = 26,
}

#[derive(..BffStruct)]
pub struct ObjectLinkHeaderV1_381_67_09PC {
    #[referenced_names(skip)]
    link_name: Name,
    data_name: Name,
    b_sphere: Sphere,
    b_box: BffBox,
    fade_out_dist: f32,
    flags: ObjectFlagsV1_381_67_09PC,
    r#type: ObjectType,
}

#[derive(..BffStruct)]
pub struct ResourceObjectLinkHeaderV1_06_63_02PC {
    #[referenced_names(skip)]
    link_name: Name,
    names: DynArray<Name>,
    #[br(parse_with = until_eof)]
    links: Vec<u8>,
}

#[derive(..BffStruct)]
pub struct ObjectLinkHeaderV1_06_63_02PC {
    #[referenced_names(skip)]
    link_name: Name,
    names: DynArray<Name>,
    data_name: Name,
    b_sphere: Sphere,
    b_box: BffBox,
    fade_out_dist: f32,
    pub flags: u32,
    r#type: ObjectType,
}
