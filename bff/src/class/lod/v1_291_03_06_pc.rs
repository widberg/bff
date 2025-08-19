use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    Cylindre,
    DynArray,
    DynBox,
    DynSphere,
    ObjectLinkHeaderV1_06_63_02PC,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct CylindreCol {
    cylindre: Cylindre,
    flag: u32,
    name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SphereColNode {
    data: [u8; 28],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ClassRes {
    id: u32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LodBodyV1_291_03_06PC {
    b_sphere_col_node: Name,
    #[br(if(b_sphere_col_node != Name::default()))]
    sphere_col_node: Option<SphereColNode>,
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylinder_cols: DynArray<CylindreCol>,
    close: Vec3f,
    component_names: DynArray<Name>,
    shadow_name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    anims: BffOption<DynArray<ClassRes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    sounds: BffOption<DynArray<ClassRes>>,
    user_define_name: Name,
}

pub type LodV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LodBodyV1_291_03_06PC>;

impl Export for LodV1_291_03_06PC {}
impl Import for LodV1_291_03_06PC {}
