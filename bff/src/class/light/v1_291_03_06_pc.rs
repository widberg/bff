use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Sphere, Vec3f, RGBA};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    links: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    unknown_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
pub struct LightBodyV1_291_03_06PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

pub type LightV1_291_03_06PC = TrivialClass<LinkInfo, LightBodyV1_291_03_06PC>;
