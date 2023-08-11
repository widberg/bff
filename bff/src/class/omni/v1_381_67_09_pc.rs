use crate::class::trivial_class::TrivialClass;
use crate::math::Mat4f;
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
pub struct OmniBodyV1_381_67_09PC {
	scale_matrix: Mat4f,
	translation_matrix: Mat4f,
	trs_mat: Mat4f,
	material_anim_name0: Name,
	material_anim_name1: Name,
}

pub type OmniV1_381_67_09PC = TrivialClass<(), OmniBodyV1_381_67_09PC>;