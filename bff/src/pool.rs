use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::dynarray::DynArray;
use crate::name::Name;
use crate::object::PoolObject;

#[derive(BinRead, Serialize, Debug, BinWrite)]
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

impl ReferenceRecord {
    pub fn objects_name_starting_index(&self) -> u32 {
        self.objects_name_starting_index
    }

    pub fn objects_name_count(&self) -> u16 {
        self.objects_name_count
    }
}

#[derive(BinRead, Serialize, Debug, BinWrite)]
pub struct ObjectDescription {
    name: Name,
    reference_count: u32,
    padded_size: u32,
    reference_records_index: u32,
}

impl ObjectDescription {
    pub fn name(&self) -> Name {
        self.name
    }

    pub fn reference_records_index(&self) -> u32 {
        self.reference_records_index
    }
}

#[derive(BinRead, Debug, BinWrite)]
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

#[derive(BinRead, Serialize, Debug, BinWrite)]
pub struct PoolHeader {
    #[serde(skip)]
    _equals524288: u32,
    #[serde(skip)]
    _equals2048: u32,
    #[serde(skip)]
    _objects_names_count_sum: u32,
    pub object_descriptions_indices: DynArray<u32>,
    #[br(map = zip_object_description_soa)]
    pub object_descriptions: Vec<ObjectDescription>,
    pub reference_records: DynArray<ReferenceRecord>,
    #[br(align_after = 2048)]
    #[serde(skip)]
    _reference_records_sentinel: ReferenceRecord,
}

#[derive(BinRead, Serialize, Debug, BinWrite)]
pub struct Pool {
    pub header: PoolHeader,
    #[br(count = header.object_descriptions_indices.len())]
    pub objects: Vec<PoolObject>,
}
