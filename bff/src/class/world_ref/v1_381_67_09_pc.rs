use crate::class::trivial_class::TrivialClass;
use crate::name::Name;
use crate::dynarray::DynArray;
use crate::math::Vec3f;

#[derive(BinRead, Debug, Serialize)]
struct UUIDPair {
	uuid0: u32,
	uuid1: u32,
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
pub struct WorldRefBodyV1_381_67_09PC {
	node_name0: Name,
	warp_name: Name,
	game_obj_name: Name,
	unused14: Name,
	gen_world_name: Name,
	node_name1: Name,
	unused17s: DynArray<u32>,
	unuseds: DynArray<u8>,
	mats: DynArray<Mat4f>,
	pointA: Vec3f,
	pointB: Vec3f,
	uuid_pairs: DynArray<UUIDPair>,
	init_script: PascalStringNULL,
	node_name2: DynArray<Name>,
	zero: u32,
}

pub type WorldRefV1_381_67_09PC = TrivialClass<(), WorldRefBodyV1_381_67_09PC>;