use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::math::{DynBox, DynSphere, Vec3f};
use crate::names::Name;
use crate::option::BffOption;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
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
    anims: BffOption<DynArray<ClassRes>>,
    sounds: BffOption<DynArray<ClassRes>>,
    user_define_crc32: Name,
}

pub type LodV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LodBodyV1_291_03_06PC>;
