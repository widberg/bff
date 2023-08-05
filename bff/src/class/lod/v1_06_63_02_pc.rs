use binrw::BinRead;
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{DynBox, DynSphere, Mat4f, Sphere, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    linked_crc32: DynArray<Name>,
    lod_data_crc32: Name,
    b_sphere_local: Sphere,
    unk_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name_crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(link_header: &LinkInfo))]
pub struct LodBodyV1_06_63_02PC {
    b_sphere_col_node: Name,
    #[br(if(b_sphere_col_node != 0))]
    sphere_col_node: Option<SphereColNode>,
    spheres_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    close: Vec3f,
    component_crc32s: DynArray<Name>,
    shadow_crc32: Name,
    anims: DynArray<ClassRes>,
    #[br(if(link_header.flags & 1048576 >= 1))]
    sounds: Option<DynArray<ClassRes>>,
    user_define_crc32: Name,
}

pub type LodV1_06_63_02PC = TrivialClass<LinkInfo, LodBodyV1_06_63_02PC>;
