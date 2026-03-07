use binrw::{BinRead, BinWrite, binrw};

use crate::bigfile::v1_06_63_02_pc::resource::Resource;
use crate::helpers::{DynArray, calculated_padded};
use crate::names::Name;

#[binrw]
#[derive(Debug, Default)]
pub struct ReferenceRecord {
    pub start_chunk_index: u32,
    pub end_chunk_index: u32,
    pub object_names_starting_index: u32,
    #[br(temp)]
    #[bw(calc = 0)]
    _placeholder_dpc_index: u16,
    pub object_names_count: u16,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct ResourceDescription {
    pub name: Name,
    pub reference_count: u32,
    pub padded_size: u32,
    pub reference_records_index: u32,
}

#[derive(BinRead, BinWrite, Debug)]
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
pub struct PoolManifest {
    #[br(temp)]
    #[br(assert(_equals0x400000 == 0x400000))]
    #[bw(calc = 0x400000)]
    _equals0x400000: u32,
    #[br(temp)]
    #[br(assert(_equals0x20 == 0x20))]
    #[bw(calc = 0x20)]
    _equals0x20: u32,
    pub object_names_count_sum: u32,
    pub object_names_indices: DynArray<u32>,
    pub object_names: DynArray<Name>,
    #[br(map = zip_resource_description_soa)]
    #[bw(map = unzip_resource_description_soa)]
    pub resource_descriptions: Vec<ResourceDescription>,
    pub object_padded_size: DynArray<u32>,
    pub reference_records_indices: DynArray<u32>,
    pub reference_records: DynArray<ReferenceRecord>,
    #[brw(align_after = 2048)]
    #[bw(calc = <_>::default())]
    _reference_records_sentinel: ReferenceRecord,
}

#[derive(BinRead, Debug)]
pub struct Pool {
    pub header: PoolManifest,
    #[br(count = header.object_names_indices.inner.len())]
    pub resources: Vec<PoolResource>,
}

#[derive(BinRead, Debug)]
pub struct PoolResource {
    #[br(align_after = 2048)]
    pub resource: Resource,
}

pub fn calculate_padded_pool_header_size(
    object_names_count: usize,
    resource_descriptions_count: usize,
    reference_records_count: usize,
) -> usize {
    let size = 4 + // equals0x400000
        4 + // equals0x20
        4 + // object_names_count_sum
        4 + // object_names_indices count
        object_names_count * 4 + // object_names_indices
        4 + // object_names count
        resource_descriptions_count * 4 + // object_names
        4 + // reference_counts count
        resource_descriptions_count * 4 + // reference_counts
        4 + // object_padded_size count
        resource_descriptions_count * 4 + // object_padded_size
        4 + // reference_records_indices count
        resource_descriptions_count * 4 + // reference_records_indices
        4 + // reference_records count
        (reference_records_count + 1) * (4 + 4 + 4 + 2 + 2); // reference records + terminal

    calculated_padded(size, 2048)
}
