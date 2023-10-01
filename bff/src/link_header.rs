use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Sphere};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct ResourceObjectLinkHeader {
    link_name: Name,
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, Deserialize)]
pub struct ObjectDatasFlagsV1_381_67_09PC {
    fl_objectdatas_hide: u1,
    fl_objectdatas_code_control: u1,
    fl_objectdatas_cloned: u1,
    fl_objectdatas_skinned: u1,
    fl_objectdatas_morphed: u1,
    fl_objectdatas_vreflect: u1,
    fl_objectdatas_hide_shadow: u1,
    fl_objectdatas_static_shadow: u1,
    fl_objectdatas_vp0_hide: u1,
    fl_objectdatas_vp1_hide: u1,
    fl_objectdatas_vp2_hide: u1,
    fl_objectdatas_vp3_hide: u1,
    fl_objectdatas_last: u1,
    padding: u19,
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, Deserialize)]
pub struct ObjectFlagsV1_381_67_09PC {
    fl_object_init: u1,
    fl_object_max_bsphere: u1,
    fl_object_skinned: u1,
    fl_object_morphed: u1,
    fl_object_orientedbbox: u1,
    fl_object_no_seaddisplay: u1,
    fl_object_no_seadcollide: u1,
    fl_object_no_display: u1,
    fl_object_transparent: u1,
    fl_object_optimized_vertex: u1,
    fl_object_linear_mapping: u1,
    fl_object_skinned_with_one_bone: u1,
    fl_object_light_baked: u1,
    fl_object_light_baked_with_material: u1,
    fl_object_shadow_receiver: u1,
    fl_object_no_tesselate: u1,
    fl_object_last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct ObjectLinkHeaderV1_381_67_09PC {
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlagsV1_381_67_09PC,
    r#type: ObjectType,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct ObjectLinkHeaderV1_06_63_02PC {
    link_crc32: Name,
    links: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    unknown_matrix: Mat4f,
    fade_out_distance: f32,
    pub flags: u32,
    r#type: u16,
}
