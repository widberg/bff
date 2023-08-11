use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;
use crate::math::Vec3f;
use crate::math::Vec2f;
use bilge::prelude::1;
use bilge::prelude::24;
use bilge::prelude::2;
use bilge::prelude::16;
use bilge::prelude::11;
type VertexVectorComponent = u8;
type VertexVector3u8 = [VertexVectorComponent;  3];
type VertexBlendIndex = f32;
type DisplacementVectorComponent = NumeratorFloat<i16, 1024>;
type ShortVecWeird = [NumeratorFloat<i16, 1024>;  3];
#[derive(BinRead, Debug, Serialize)]
pub struct Points {
	//FIXME: inherits Object_Z
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &Points))]
pub struct Mesh_LinkHeaderBodyV1_381_67_09PC {
	names: DynArray<Name>,
	fade: FadeDistances,
	dyn_spheres: DynArray<DynSphere>,
	dyn_boxes: DynArray<DynBox>,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused0 {
	unknown0: u32,
	unknown1: u32,
	unknown2: u32,
	unknown3: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Strip {
	strip_vertices_indices: DynArray<u16>,
	material_name: Name,
	tri_order: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused00 {
	unused0: u32,
	unused1: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused4 {
	unused0s: DynArray<Unused00>,
}

#[derive(BinRead, Debug, Serialize)]
struct CollisionAABB {
	min: Vec3f,
	collision_aabb_range: RangeBeginEnd,
	max: Vec3f,
	collision_faces_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct CollisionFace {
	short_vec_weirds_indices: [u16; 3],
	surface_type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused8 {
	unused0: u32,
	unused1: u32,
	unused2: u32,
	unused3: u32,
	unused4: u32,
	unused5: u32,
	unused6: u32,
	unused7: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayoutPosition {
	position: Vec3f,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayoutNoBlend {
	position: Vec3f,
	tangent: VertexVector3u8,
	tangent_w: VertexVectorComponent,
	normal: VertexVector3u8,
	normal_w: VertexVectorComponent,
	uv: Vec2f,
	luv: Vec2f,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayout1Blend {
	position: Vec3f,
	tangent: VertexVector3u8,
	tangent_w: VertexVectorComponent,
	normal: VertexVector3u8,
	normal_w: VertexVectorComponent,
	uv: Vec2f,
	blend_index: VertexBlendIndex,
	pad2: [i32; 3],
	blend_weight: f32,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayout4Blend {
	position: Vec3f,
	tangent: VertexVector3u8,
	tangent_w: VertexVectorComponent,
	normal: VertexVector3u8,
	normal_w: VertexVectorComponent,
	uv: Vec2f,
	blend_indices: [VertexBlendIndex; 4],
	blend_weights: [f32; 4],
}

#[bitsize(32)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum VertexLayout {
	VertexLayoutPosition = 12,
	VertexLayoutNoBlend = 36,
	VertexLayout1Blend = 48,
	VertexLayout4Blend = 60,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(4)]
struct D3DPOOL {
	D3DPOOL_DEFAULT: u1,
	D3DPOOL_MANAGED: u1,
	D3DPOOL_SYSTEMMEM: u1,
	D3DPOOL_SCRATCH: u1,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(28)]
struct D3DUSAGE {
	D3DUSAGE_DYNAMIC: u1,
	D3DUSAGE_WRITEONLY: u1,
	#[br(temp)]	padding: u1,
	UNKNOWN: u1,
	#[br(temp)]	padding: u24,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(0)]
struct D3DFlags {
	D3DPOOL: D3DPOOL,
	D3DUSAGE: D3DUSAGE,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexBufferExt {
	vertex_count: u32,
	vertex_layout: VertexLayout,
	flags: D3DFlags,
	//FIXME: match (vertex_layout) {
	//FIXME: (VertexLayout::VertexLayoutPosition): VertexLayoutPosition vertices[vertex_count];
	//FIXME: (VertexLayout::VertexLayoutNoBlend): VertexLayoutNoBlend vertices[vertex_count];
	//FIXME: (VertexLayout::VertexLayout1Blend): VertexLayout1Blend vertices[vertex_count];
	//FIXME: (VertexLayout::VertexLayout4Blend): VertexLayout4Blend vertices[vertex_count];
	//FIXME: }
}

#[derive(BinRead, Debug, Serialize)]
struct IndexBufferExt {
	index_count: u32,
	flags: D3DFlags,
	#[br(count = index_count)]	data: Vec<u16>,
}

#[derive(BinRead, Debug, Serialize)]
struct Quad {
	vertices: [Vec3f; 4],
	normal: Vec3f,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused1 {
	unused0: u32,
	unused1: u32,
	unused2: u32,
	unused3: u32,
	unused4: u32,
	unused5: u32,
	unused6: u32,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(31)]
struct VertexGroupFlags {
	#[br(temp)]	padding: u2,
	VISIBLE: u1,
	#[br(temp)]	padding: u16,
	MORPH: u1,
	#[br(temp)]	padding: u11,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexGroup {
	vertex_buffer_index: u32,
	index_buffer_index: u32,
	quad_range: RangeBeginSizeU32,
	flags: VertexGroupFlags,
	vertex_buffer_range: RangeBeginEnd,
	vertex_count: u32,
	index_buffer_index_begin: u32,
	face_count: u32,
	zero: u32,
	vertex_buffer_range_begin_or_zero: u32,
	vertex_layout: u16,
	material_index: i16,
	unused1s: DynArray<Unused1>,
}

#[derive(BinRead, Debug, Serialize)]
struct AABBMorphTrigger {
	min: Vec3f,
	aabb_morph_triggers_range: RangeBeginEnd,
	max: Vec3f,
	map_index_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct DisplacementVector {
	displacement: [DisplacementVectorComponent;  3],
	displacement_vectors_self_index: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct MorphTargetDesc {
	name: PascalString,
	base_vertex_buffer_id: u32,
	displacement_vertex_buffer_index: u16,
	displacement_vectors_indicies: DynArray<u16>,
	displacement_vectors: DynArray<DisplacementVector>,
}

#[derive(BinRead, Debug, Serialize)]
struct Morpher {
	aabb_morph_triggers: DynArray<AABBMorphTrigger>,
	map: BffMap<u16, u16>,
	displacement_vectors_indices: DynArray<u16>,
	morphs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize)]
struct MeshBuffers {
	vertex_buffers: DynArray<VertexBufferExt>,
	index_buffers: DynArray<IndexBufferExt>,
	quads: DynArray<Quad>,
	vertex_groups: DynArray<VertexGroup>,
	morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize)]
struct Mesh {
	//FIXME: inherits Mesh_Z_LinkHeader
	strip_vertices: DynArray<Vec3f>,
	unused0s: DynArray<Unused0>,
	texcoords: DynArray<Vec2f>,
	normals: DynArray<Vec3f>,
	strips: DynArray<Strip>,
	unused4s: DynArray<Unused4>,
	material_names: DynArray<Name>,
	collision_aabbs: DynArray<CollisionAABB>,
	collision_faces: DynArray<CollisionFace>,
	unused8s: DynArray<Unused8>,
	mesh_buffers: MeshBuffers,
	short_vec_weirds: DynArray<ShortVecWeird>,
}

pub type MeshV1_381_67_09PC = TrivialClass<(), Mesh>;