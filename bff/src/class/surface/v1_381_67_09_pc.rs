use crate::class::trivial_class::TrivialClass;
use crate::math::Mat4f;
use crate::name::Name;
use bilge::prelude::3;
use bilge::prelude::5;
use bilge::prelude::8;
use serde_big_array::BigArray;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
struct Unused2 {
	data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct Unused3 {
	data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct Patch {
	flag: u16,
	index_in_unk_short_da: u16,
	edge_indices: [u16; 4],
	material_anim_index: u32,
	data: [u32; 12],
	mat: Mat4f,
	vec4fs_indices: [u16; 4],
	unknown3s: [u16; 17],
	surface_indices_index: u16,
	material_anim_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct Edge {
	P: [u16; 2],
	T: [u16; 2],
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(16)]
struct ShouldDrawBitfield {
	index_in_draw_info_array: u3,
	shift_amount_for_bit: u5,
	other: u8,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused12 {
	data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct SeadVoxel {
	patches_indices_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown15 {
	#[serde(with = "BigArray")]	data: [u32; 48],
	sead_voxel_count: u32,
	patch_count_related: u32,
	unknown0s: [u32; 2],
}

#[derive(BinRead, Debug, Serialize)]
struct SeadIndex {
	sead_voxels: DynArray<SeadVoxel>,
	patches_indices: DynArray<u16>,
	unknown15: Unknown15,
	patch_count: u32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct Points {
	//FIXME: inherits Object_Z
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &Points))]
pub struct SurfaceBodyV1_381_67_09PC {
	points: DynArray<Vec3f>,
	vec4fs: DynArray<Vec4f>,
	unused2s: DynArray<Unused2>,
	unused3s: DynArray<Unused3>,
	patches: DynArray<Patch>,
	edges: DynArray<Edge>,
	normals: DynArray<Vec3f>,
	vertex9s: DynArray<Vec3f>,
	vertex10s: DynArray<Vec2f>,
	should_draw_relateds: DynArray<ShouldDrawBitfield>,
	unused12s: DynArray<Unused12>,
	sead_index: BffOption<SeadIndex>,
}

pub type SurfaceV1_381_67_09PC = TrivialClass<(), SurfaceBodyV1_381_67_09PC>;