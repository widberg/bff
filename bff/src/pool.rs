use binrw::BinRead;
use serde::Serialize;

use crate::dynarray::DynArray;
use crate::name::Name;
use crate::object::PoolObject;

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
pub struct ObjectDescription {
    name: Name,
    reference_count: u32,
    padded_size: u32,
    reference_records_index: u32,
}

#[derive(BinRead, Debug)]
pub struct ObjectDescriptionSOA {
    names: DynArray<Name>,
    reference_counts: DynArray<u32>,
    padded_sizes: DynArray<u32>,
    reference_records_indices: DynArray<u32>,
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

#[derive(BinRead, Serialize, Debug)]
pub struct PoolHeader {
    #[serde(skip)]
    _equals524288: u32,
    #[serde(skip)]
    _equals2048: u32,
    #[serde(skip)]
    _objects_names_count_sum: u32,
    object_descriptions_indices: DynArray<u32>,
    #[br(map = zip_object_description_soa)]
    object_descriptions: Vec<ObjectDescription>,
    reference_records: DynArray<ReferenceRecord>,
    #[br(align_after = 2048)]
    #[serde(skip)]
    _reference_records_sentinel: ReferenceRecord,
}

#[derive(BinRead, Serialize, Debug)]
pub struct Pool {
    header: PoolHeader,
    #[br(count = header.object_descriptions.len())]
    objects: Vec<PoolObject>,
}
