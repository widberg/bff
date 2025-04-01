use std::default::Default;

use binrw::{BinRead, BinWrite, binrw};
use serde::Serialize;

use super::object::PoolObject;
use crate::helpers::{DynArray, calculated_padded};
use crate::names::Name;

#[binrw]
#[derive(Serialize, Debug, Default)]
pub struct ReferenceRecord {
    pub start_chunk_index: u32,
    pub end_chunk_index: u32,
    pub objects_name_starting_index: u32,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0)]
    _placeholder_bigfile_index: u16,
    pub objects_name_count: u16,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_times_referenced: u32,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_current_references_shared: u32,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_current_references_weak: u32,
}

#[derive(BinRead, Serialize, Debug, BinWrite)]
pub struct ObjectDescription {
    pub name: Name,
    pub reference_count: u32,
    pub padded_size: u32,
    pub reference_records_index: u32,
}

#[derive(BinRead, Debug, BinWrite)]
pub struct ObjectDescriptionSOA {
    names: DynArray<Name>,
    reference_counts: DynArray<u32>,
    padded_sizes: DynArray<u32>,
    reference_records_indices: DynArray<u32>,
}

fn unzip_object_description_soa(
    object_descriptions: &Vec<ObjectDescription>,
) -> ObjectDescriptionSOA {
    let mut names = Vec::new();
    let mut reference_counts = Vec::new();
    let mut padded_sizes = Vec::new();
    let mut reference_records_indices = Vec::new();

    for object_description in object_descriptions {
        names.push(object_description.name);
        reference_counts.push(object_description.reference_count);
        padded_sizes.push(object_description.padded_size);
        reference_records_indices.push(object_description.reference_records_index);
    }

    ObjectDescriptionSOA {
        names: names.into(),
        reference_counts: reference_counts.into(),
        padded_sizes: padded_sizes.into(),
        reference_records_indices: reference_records_indices.into(),
    }
}

fn zip_object_description_soa(
    object_description_soa: ObjectDescriptionSOA,
) -> Vec<ObjectDescription> {
    assert_eq!(
        object_description_soa.names.len(),
        object_description_soa.reference_counts.len()
    );
    assert_eq!(
        object_description_soa.names.len(),
        object_description_soa.padded_sizes.len()
    );
    assert_eq!(
        object_description_soa.names.len(),
        object_description_soa.reference_records_indices.len()
    );

    let mut result = Vec::with_capacity(object_description_soa.names.len());

    for i in 0..object_description_soa.names.len() {
        result.push(ObjectDescription {
            name: object_description_soa.names[i],
            reference_count: object_description_soa.reference_counts[i],
            padded_size: object_description_soa.padded_sizes[i],
            reference_records_index: object_description_soa.reference_records_indices[i],
        })
    }

    result
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct PoolHeader {
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0x80000)]
    _equals524288: u32,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0x800)]
    _equals2048: u32,
    #[serde(skip)]
    pub objects_names_count_sum: u32,
    pub object_descriptions_indices: DynArray<u32>,
    #[br(map = zip_object_description_soa)]
    #[bw(map = unzip_object_description_soa)]
    pub object_descriptions: Vec<ObjectDescription>,
    pub reference_records: DynArray<ReferenceRecord>,
    #[br(align_after = 2048)]
    #[serde(skip)]
    #[bw(calc = <_>::default())]
    _reference_records_sentinel: ReferenceRecord,
}

pub fn calculate_padded_pool_header_size(
    object_descriptions_indices_size: usize,
    object_descriptions_size: usize,
    reference_records_size: usize,
) -> usize {
    let size =
        // equals524288
        4
        // equals2048
        + 4
        // objectsCRC32CountSum
        + 4
        // objectsCRC32s
        + 4
        + object_descriptions_indices_size * 4
        // crc32s, referenceCounts, paddedSizes, referenceRecordsIndices
        + 4 * (4 + object_descriptions_size * 4)
        // referenceRecords
        + 4
        + (reference_records_size + 1) * (4 + 4 + 4 + 2 + 2 + 4 + 4 + 4);
    calculated_padded(size, 2048)
}

#[derive(BinRead, Serialize, Debug)]
pub struct Pool {
    pub header: PoolHeader,
    #[br(count = header.object_descriptions_indices.len())]
    pub objects: Vec<PoolObject>,
}
