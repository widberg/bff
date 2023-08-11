use crate::class::trivial_class::TrivialClass;
use crate::math::Quat;
use crate::math::Vec3f;
use crate::math::Vec4f;

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
pub struct LightBodyV1_381_67_09PC {
	rotation: Quat,
	direction: Vec3f,
	color: Vec4f,
	ambient: Vec3f,
	position: Vec3f,
}

pub type LightV1_381_67_09PC = TrivialClass<(), LightBodyV1_381_67_09PC>;