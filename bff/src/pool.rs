use binrw::BinRead;
use serde::Serialize;

use crate::{dynarray::DynArray, name::Name, object::PoolObject};

#[derive(BinRead, Serialize, Debug)]
pub struct ReferenceRecord {
    start_chunk_index: u32,
    end_chunk_index: u32,
    objects_name_starting_index: u32,
    #[serde(skip)]
    _placeholder_bigfile_index: u16,
    objects_name_count: u16,
    #[serde(skip)]
    _placeholder_times_referenced: u32,
    #[serde(skip)]
    _placeholder_current_references_shared: u32,
    #[serde(skip)]
    _placeholder_current_references_weak: u32,
}

#[derive(BinRead, Serialize, Debug)]
pub struct PoolHeader {
    #[serde(skip)]
    _equals524288: u32,
    #[serde(skip)]
    _equals2048: u32,
    #[serde(skip)]
    _objects_names_count_sum: u32,
    object_names_indices: DynArray<u32>,
    object_names: DynArray<Name>,
    reference_counts: DynArray<u32>,
    object_padded_size: DynArray<u32>,
    reference_records_indices: DynArray<u32>,
    reference_records: DynArray<ReferenceRecord>,
    #[br(align_after = 2048)]
    #[serde(skip)]
    _reference_records_sentinel: ReferenceRecord,
}

#[derive(BinRead, Serialize, Debug)]
pub struct Pool {
    header: PoolHeader,
    #[br(count = header.object_names.len())]
    objects: Vec<PoolObject>,
}
