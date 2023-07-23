use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{DynBox, DynSphere, Mat4f, Sphere, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    links: DynArray<Name>,
    lod_data_crc32: Name,
    b_sphere_local: Sphere,
    unknown_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct CylindreCol {
    #[br(count = 40)]
    data: Vec<u8>,
    name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
pub struct LodBodyV1_291_03_06PC {
    b_sphere_col_node: Name,
    #[br(if(b_sphere_col_node != 0))]
    sphere_col_node: Option<SphereColNode>,
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylinder_cols: DynArray<CylindreCol>,
    close: Vec3f,
    component_crc32s: DynArray<Name>,
    shadow_crc32: Name,
    has_anim: u8,
    #[br(if(has_anim == 1))]
    anims: Option<DynArray<ClassRes>>,
    has_sound: u8,
    #[br(if(has_sound == 1))]
    sounds: Option<DynArray<ClassRes>>,
    user_define_crc32: Name,
}

pub type LodV1_291_03_06PC = TrivialClass<LinkInfo, LodBodyV1_291_03_06PC>;
