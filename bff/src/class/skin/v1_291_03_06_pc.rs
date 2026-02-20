
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffMap, DynArray, ObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct Unknown1 {
    unknown1: [u8; 8],
}

#[derive(..BffStruct)]
struct BlendRelated {
    index: u32,
    blend: f32,
}

#[derive(..BffStruct)]
struct ResourceBlend {
    unknown: u16,
    blend_related1s: DynArray<BlendRelated>,
    blend_related2s: DynArray<BlendRelated>,
}

#[derive(..BffStruct)]
struct Bone {
    bone_name: Name,
    resource_blends: DynArray<ResourceBlend>,
}

#[derive(..BffStruct)]
struct MorphPacketDA {
    size_capacity: u32,
    ptr: u32,
}

#[derive(..BffStruct)]
struct MorphPacket {
    unknown0_name: Name,
    unknown1_name: Name,
}

#[derive(..BffStruct)]
pub struct SkinSubSection {
    pub material_name: Name,
    bone_node_names: [Name; 7],
    placeholder_morph_packet_da: MorphPacketDA,
    morph_packets: DynArray<MorphPacket>,
}

#[derive(..BffStruct)]
pub struct SkinSection {
    pub skin_sub_sections: DynArray<SkinSubSection>,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct SkinBodyV1_291_03_06PC {
    pub mesh_names: DynArray<Name>,
    unknown0s: DynArray<Unknown1>,
    bones: DynArray<Bone>,
    is_class_id: u8,
    #[br(if(is_class_id != 0))]
    anim_class_ids: Option<BffMap<i32, i32>>,
    #[br(if(is_class_id != 0))]
    sound_class_ids: Option<BffMap<i32, i32>>,
    matrix_cache_check: u32,
    pub skin_sections: DynArray<SkinSection>,
}

pub type SkinV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, SkinBodyV1_291_03_06PC>;

impl Export for SkinV1_291_03_06PC {}
impl Import for SkinV1_291_03_06PC {}
