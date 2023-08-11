use crate::class::trivial_class::TrivialClass;
use serde_big_array::BigArray;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
struct BlockDescription {
	object_count: u32,
	padded_size: u32,
	data_size: u32,
	working_buffer_offset: u32,
	first_object_name: u32,
	zero: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Header {
	#[serde(with = "BigArray")]	version: [char; 256],
	is_not_rtc: u32,
	block_count: u32,
	block_working_buffer_capacity_even: u32,
	block_working_buffer_capacity_odd: u32,
	padded_size: u32,
	version_patch: u32,
	version_minor: u32,
	version_major: u32,
	#[br(count = block_count)]	block_descriptions: Vec<BlockDescription>,
	#[br(count = 64 - block_count)]	empty_block_descriptions: Vec<BlockDescription>,
	pool_manifest_padded_size: u32,
	pool_manifest_offset: u32,
	pool_manifest_unused0: u32,
	pool_manifest_unused1: u32,
	pool_object_decompression_buffer_capacity: u32,
	block_sector_padding_size: u32,
	pool_sector_padding_size: u32,
	file_size: u32,
	#[serde(with = "BigArray")]	incredi_builder_string: [char; 128],
	#[serde(with = "BigArray")]	pad: [u8; 64],
}

#[derive(BinRead, Debug, Serialize)]
struct ObjectHeader {
	data_size: u32,
	link_header_size: u32,
	decompressed_size: u32,
	compressed_size: u32,
	class_name: u32,
	object_name: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct BlockObject {
	header: ObjectHeader,
	#[br(count = header.data_size)]	data: Vec<u8>,
}

#[derive(BinRead, Debug, Serialize)]
struct Block {
	#[br(count = parent.header.block_descriptions)]	objects: Vec<BlockObject>,
}

#[derive(BinRead, Debug, Serialize)]
struct ReferenceRecord {
	start_chunk_index: u32,
	end_chunk_index: u32,
	object_names_starting_index: u32,
	placeholder_dpc_index: u16,
	object_names_count: u16,
	placeholder_times_referenced: u32,
	placeholder_current_references_shared: u32,
	placeholder_current_references_weak: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct PoolManifest {
	equals0x80000: u32,
	equals0x800: u32,
	object_names_count_sum: u32,
	object_names_indices: DynArray<u32>,
	object_names: DynArray<Name>,
	reference_counts: DynArray<u32>,
	object_padded_size: DynArray<u32>,
	reference_records_indices: DynArray<u32>,
	reference_records: DynArray<ReferenceRecord>,
	terminal: ReferenceRecord,
}

#[derive(BinRead, Debug, Serialize)]
struct PoolObject {
	//FIXME: inherits BlockObject
}

#[derive(BinRead, Debug, Serialize)]
struct Pool {
	pool_manifest: PoolManifest,
	#[br(count = pool_manifest.object_names_indices.size)]	pool_objects: Vec<PoolObject>,
}

#[derive(BinRead, Debug, Serialize)]
struct DPC {
	header: Header,
	#[br(count = header.block_count)]	blocks: Vec<Block>,
	//FIXME: if (header.pool_manifest_padded_size != 0) {
	//FIXME: Pool pool;
	//FIXME: }
}

pub type DPCV1_381_67_09PC = TrivialClass<(), DPC>;