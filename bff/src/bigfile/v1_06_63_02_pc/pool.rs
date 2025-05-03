use std::default::Default;

use binrw::{BinRead, BinWrite, binrw};

use super::resource::PoolResource;
use crate::helpers::{DynArray, calculated_padded};
use crate::names::Name;

#[binrw]
#[derive(Debug, Default)]
pub struct ReferenceRecord {
    pub start_chunk_index: u32,
    pub end_chunk_index: u32,
    pub resources_name_starting_index: u32,
    #[br(temp)]
    #[bw(calc = 0)]
    _placeholder_bigfile_index: u16,
    pub resources_name_count: u16,
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_times_referenced: u32,
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_current_references_shared: u32,
    #[br(temp)]
    #[bw(calc = 0xFFFFFFFF)]
    _placeholder_current_references_weak: u32,
}

#[derive(BinRead, Debug, BinWrite)]
pub struct ResourceDescription {
    pub name: Name,
    pub reference_count: u32,
    pub padded_size: u32,
    pub reference_records_index: u32,
}

#[derive(BinRead, Debug, BinWrite)]
pub struct ResourceDescriptionSOA {
    names: DynArray<Name>,
    reference_counts: DynArray<u32>,
    padded_sizes: DynArray<u32>,
    reference_records_indices: DynArray<u32>,
}

fn unzip_resource_description_soa(
    resource_descriptions: &Vec<ResourceDescription>,
) -> ResourceDescriptionSOA {
    let mut names = Vec::new();
    let mut reference_counts = Vec::new();
    let mut padded_sizes = Vec::new();
    let mut reference_records_indices = Vec::new();

    for resource_description in resource_descriptions {
        names.push(resource_description.name);
        reference_counts.push(resource_description.reference_count);
        padded_sizes.push(resource_description.padded_size);
        reference_records_indices.push(resource_description.reference_records_index);
    }

    ResourceDescriptionSOA {
        names: names.into(),
        reference_counts: reference_counts.into(),
        padded_sizes: padded_sizes.into(),
        reference_records_indices: reference_records_indices.into(),
    }
}

fn zip_resource_description_soa(
    resource_description_soa: ResourceDescriptionSOA,
) -> Vec<ResourceDescription> {
    assert_eq!(
        resource_description_soa.names.len(),
        resource_description_soa.reference_counts.len()
    );
    assert_eq!(
        resource_description_soa.names.len(),
        resource_description_soa.padded_sizes.len()
    );
    assert_eq!(
        resource_description_soa.names.len(),
        resource_description_soa.reference_records_indices.len()
    );

    let mut result = Vec::with_capacity(resource_description_soa.names.len());

    for i in 0..resource_description_soa.names.len() {
        result.push(ResourceDescription {
            name: resource_description_soa.names[i],
            reference_count: resource_description_soa.reference_counts[i],
            padded_size: resource_description_soa.padded_sizes[i],
            reference_records_index: resource_description_soa.reference_records_indices[i],
        })
    }

    result
}

#[binrw]
#[derive(Debug)]
pub struct PoolHeader {
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0x80000)]
    _equals524288: u32,
    #[serde(skip)]
    #[br(temp)]
    #[bw(calc = 0x800)]
    _equals2048: u32,
    pub resources_names_count_sum: u32,
    pub resource_descriptions_indices: DynArray<u32>,
    #[br(map = zip_resource_description_soa)]
    #[bw(map = unzip_resource_description_soa)]
    pub resource_descriptions: Vec<ResourceDescription>,
    pub reference_records: DynArray<ReferenceRecord>,
    #[br(align_after = 2048)]
    #[serde(skip)]
    #[bw(calc = <_>::default())]
    _reference_records_sentinel: ReferenceRecord,
}

pub fn calculate_padded_pool_header_size(
    resource_descriptions_indices_size: usize,
    resource_descriptions_size: usize,
    reference_records_size: usize,
) -> usize {
    let size =
        // equals524288
        4
        // equals2048
        + 4
        // resourcesCRC32CountSum
        + 4
        // resourcesCRC32s
        + 4
        + resource_descriptions_indices_size * 4
        // crc32s, referenceCounts, paddedSizes, referenceRecordsIndices
        + 4 * (4 + resource_descriptions_size * 4)
        // referenceRecords
        + 4
        + (reference_records_size + 1) * (4 + 4 + 4 + 2 + 2 + 4 + 4 + 4);
    calculated_padded(size, 2048)
}

#[derive(BinRead, Debug)]
pub struct Pool {
    pub header: PoolHeader,
    #[br(count = header.resource_descriptions_indices.len())]
    pub resources: Vec<PoolResource>,
}
