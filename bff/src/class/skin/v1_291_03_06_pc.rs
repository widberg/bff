use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::map::BffMap;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Unknown1 {
    unknown1: [u8; 8],
}

#[derive(BinRead, Debug, Serialize)]
struct BlendRelated {
    index: u32,
    blend: f32,
}

#[derive(BinRead, Debug, Serialize)]
struct ObjectBlend {
    unknown: u16,
    blend_related1s: DynArray<BlendRelated>,
    blend_related2s: DynArray<BlendRelated>,
}

#[derive(BinRead, Debug, Serialize)]
struct Bone {
    bone_name_crc32: Name,
    object_blends: DynArray<ObjectBlend>,
}

#[derive(BinRead, Debug, Serialize)]
struct MorphPacketDA {
    size_capacity: u32,
    ptr: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct MorphPacket {
    unknown0_crc32: Name,
    unknown1_crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct SkinSubSection {
    material_crc32: Name,
    bone_node_name_crc32s: [Name; 7],
    placeholder_morph_packet_da: MorphPacketDA,
    morph_packets: DynArray<MorphPacket>,
}

#[derive(BinRead, Debug, Serialize)]
struct SkinSection {
    skin_sub_sections: DynArray<SkinSubSection>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct SkinBodyV1_291_03_06PC {
    mesh_crc32s: DynArray<Name>,
    unknown0s: DynArray<Unknown1>,
    bones: DynArray<Bone>,
    is_class_id: u8,
    #[br(if(is_class_id != 0))]
    anim_class_ids: Option<BffMap<i32, i32>>,
    #[br(if(is_class_id != 0))]
    sound_class_ids: Option<BffMap<i32, i32>>,
    matrix_cache_check: u32,
    skin_sections: DynArray<SkinSection>,
}

pub type SkinV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, SkinBodyV1_291_03_06PC>;
