use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, DynBox, DynSphere, ObjectLinkHeaderV1_06_63_02PC, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LodBodyV1_06_63_02PC {
    b_sphere_col_node: Name,
    #[br(if(b_sphere_col_node != Name::default()))]
    sphere_col_node: Option<SphereColNode>,
    spheres_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    close: Vec3f,
    component_names: DynArray<Name>,
    shadow_name: Name,
    anims: DynArray<ClassRes>,
    #[br(if(link_header.flags & 0x100000 != 0))]
    sounds: Option<DynArray<ClassRes>>,
    user_define_name: Name,
}

pub type LodV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LodBodyV1_06_63_02PC>;

impl Export for LodV1_06_63_02PC {}
impl Import for LodV1_06_63_02PC {}
