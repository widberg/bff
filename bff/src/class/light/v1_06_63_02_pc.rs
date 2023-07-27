use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Sphere};
use crate::name::Name;
#[derive(BinRead, Debug, Serialize)]
struct Rect {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct LinkInfo {
    link_crc32: Name,
    linked_crc32: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    unk_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct ObjectHeader {
    data_size: u32,
    link_size: u32,
    decompressed_size: u32,
    compressed_size: u32,
    class_crc32: Name,
    name_crc32: Name,
    link_info: LinkInfo,
}

#[derive(BinRead, Debug, Serialize)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LightBodyV1_06_63_02PC {
    obj_header: ObjectHeader,
    rotation: Quat,
    direction: Vec3f,
    color: Color,
    ambient: Vec3f,
    pos: Vec3f,
}

pub type LightV1_06_63_02PC = TrivialClass<LinkInfo, LightBodyV1_06_63_02PC>;
