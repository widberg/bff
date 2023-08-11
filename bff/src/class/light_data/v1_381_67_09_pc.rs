use crate::class::trivial_class::TrivialClass;
use crate::math::Vec3f;

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
	//FIXME: inherits ResourceObject_Z
	flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct LightDataBodyV1_381_67_09PC {
	facing: Vec3f,
	local_collision_sphere: Vec3f,
	unused_vec: Vec3i32,
	flags: u32,
	local_collision_sphere_facing: Vec3f,
}

pub type LightDataV1_381_67_09PC = TrivialClass<(), LightDataBodyV1_381_67_09PC>;