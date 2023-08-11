use crate::class::trivial_class::TrivialClass;
use crate::math::Mat4f;
use crate::dynarray::DynArray;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct CollisionVolInfo {
	local_transform: Mat4f,
	local_transform_inverse: Mat4f,
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
pub struct CollisionVolBodyV1_381_67_09PC {
	collision_vol_info: DynArray<CollisionVolInfo>,
	in_message_id: Name,
	out_message_id: Name,
	node_name_params: [Name; 12],
	float_params: [f32; 12],
	anim_frame_names: DynArray<Name>,
	material_anim_names: DynArray<Name>,
	volume_type: Name,
	delay: f32,
}

pub type CollisionVolV1_381_67_09PC = TrivialClass<(), CollisionVolBodyV1_381_67_09PC>;