use crate::class::trivial_class::TrivialClass;
use crate::name::Name;

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
pub struct CameraBodyV1_381_67_09PC {
	angle_of_view: f32,
	zero: f32,
	node_name: Name,
}

pub type CameraV1_381_67_09PC = TrivialClass<(), CameraBodyV1_381_67_09PC>;