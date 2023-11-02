use serde::Serialize;

use crate::names::Name;
use crate::platforms::Platform;
use crate::versions::{Version, VersionXple};

#[derive(Serialize, Debug)]
pub struct ManifestPoolObjectEntry {
    pub name: Name,
    pub reference_record_index: u32,
}

#[derive(Serialize, Debug)]
pub struct ManifestPoolReferenceRecord {
    pub object_entries_starting_index: u32,
    pub object_entries_count: u16,
}

#[derive(Serialize, Debug)]
pub struct ManifestPool {
    pub object_entry_indices: Vec<u32>,
    pub object_entries: Vec<ManifestPoolObjectEntry>,
    pub reference_records: Vec<ManifestPoolReferenceRecord>,
}

#[derive(Serialize, Debug)]
pub struct ManifestObject {
    pub name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct ManifestBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    pub objects: Vec<ManifestObject>,
}

#[derive(Serialize, Debug)]
pub struct Manifest {
    pub version: Version,
    pub platform: Platform,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_xple: Option<VersionXple>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_manifest_unused: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incredi_builder_string: Option<String>,
    pub blocks: Vec<ManifestBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<ManifestPool>,
}
