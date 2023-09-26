use bilge::prelude::*;
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ResourceObjectLinkHeader;
use crate::math::{Vec2f, RGB};
use crate::name::Name;

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits)]
struct MaterialEnabledBitmaps {
    diffuse: u1,
    unused0: u1,
    specular: u1,
    add_normal_local: u1,
    occlusion: u1,
    normal: u1,
    dirt: u1,
    normal_local: u1,
    unused1: u1,
    padding: u23,
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits)]
struct MaterialRdrFlags {
    padding0: u5,
    transparency: u1,
    padding1: u26,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct MaterialBodyV1_381_67_09PC {
    diffuse: RGB,
    opacity: f32,
    emission: RGB,
    unknown0: i32,
    t_matrix_top_left: Vec2f,
    some_number: i32,
    flags1: u32,
    t_matrix_bottom_right: Vec2f,
    enabled_bitmaps: MaterialEnabledBitmaps,
    rdr_flag: MaterialRdrFlags,
    t_matrix_offset: Vec2f,
    t_matrix_scale: f32,
    t_rotation: f32,
    specular: RGB,
    alpha_ref: f32,
    env_map_factor: f32,
    spec_map_factor: f32,
    bump_map_factor: f32,
    param4: f32,
    t_translation: Vec2f,
    t_scale: Vec2f,
    something_bitmap_related: Vec2f,
    s_diffuse_bitmap_name: Name,
    unused_bitmap_name0: Name,
    s_specular_bitmap_name: Name,
    s_add_normal_local_bitmap_name1: Name,
    s_occlusion_bitmap_name: Name,
    s_normal_bitmap_name: Name,
    s_dirt_bitmap_name: Name,
    s_normal_local_bitmap_name: Name,
    unused_bitmap_name1: Name,
}

pub type MaterialV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, MaterialBodyV1_381_67_09PC>;
