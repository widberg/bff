use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Mat4f;
use crate::math::Vec3f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Category {
	one: u32,
	node_name_arrays: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize)]
struct cA_FlatSurface {
	zero0: u32,
	mat: Mat4f,
	a: f32,
	b: f32,
	c: f32,
	reciprocal: f32,
	vec: Vec3f,
	unknown1: f32,
	unknown3: f32,
	zero1: u32,
	zero2: u32,
	zero3: u32,
	zero4: u32,
	unknown9: i32,
	unknown4: u8,
	unknown20: u8,
	unknown21: u8,
	unknown22: u8,
	unknown23: u8,
	unknown24: u8,
	unknown2: u8,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused10 {
	unused0: u32,
	unused1s: [u32; 8],
	unused2: u32,
	unused3: u32,
	unused4: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct RegionEdge {
	region_vertices_index_a: u32,
	region_vertices_index_b: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Region {
	unknown: u8,
	region_edges_indices: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct Object {
	//FIXME: inherits ResourceObject_Z
	data_name: Name,
	rot: Quat,
	transform: Mat4f,
	radius: f32,
	flags: ObjectFlags,
	type: ObjectType,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &Object))]
pub struct GenWorldBodyV1_381_67_09PC {
	node_name: Name,
	user_define_name: Name,
	gw_road_name: Name,
	binary_names: DynArray<Name>,
	bitmap_names: DynArray<Name>,
	material_names: DynArray<Name>,
	equals41: u32,
	categories: BffMap<PascalStringNULL, Category>,
	cA_FlatSurfaces: DynArray<cA_FlatSurface>,
	cancel_object_placement: DynArray<Mat4f>,
	unused10s: DynArray<Unused10>,
	region_vertices: DynArray<Vec2f>,
	region_edges: DynArray<RegionEdge>,
	regions: BffMap<FixedStringNULL<31>, Region>,
}

pub type GenWorldV1_381_67_09PC = TrivialClass<(), GenWorldBodyV1_381_67_09PC>;