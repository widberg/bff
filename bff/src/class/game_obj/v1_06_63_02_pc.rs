use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;
#[derive(BinRead, Debug, Serialize)]
struct LinkInfo {
    link_crc32: Name,
    linked_crc32: DynArray<Name>,
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
pub struct GameObjBodyV1_06_63_02PC {
    object_header: ObjectHeader,
    node_crc32s: DynArray<Name>,
}

pub type GameObjV1_06_63_02PC = TrivialClass<LinkInfo, GameObjBodyV1_06_63_02PC>;
