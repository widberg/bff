use crate::class::trivial_class::TrivialClass;
use crate::math::Vec3f;
use crate::name::Name;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct WarpBodyV1_381_67_09PC {
	flags: u32,
	vertices: [Vec3f; 8],
	one: f32,
	zero: u32,
	radius: f32,
	material_anims: [Name; 6],
	node: Name,
	anim_frames: DynArray<Name>,
}

pub type WarpV1_381_67_09PC = TrivialClass<(), WarpBodyV1_381_67_09PC>;