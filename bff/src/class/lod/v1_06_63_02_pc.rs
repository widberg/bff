use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::math::{DynBox, DynSphere, Vec3f};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name_crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
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
    #[br(if(link_header.flags & 0x100000 != 0))]
    sounds: Option<DynArray<ClassRes>>,
    user_define_crc32: Name,
}

pub type LodV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LodBodyV1_06_63_02PC>;
