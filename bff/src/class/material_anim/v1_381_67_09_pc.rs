use crate::class::trivial_class::TrivialClass;
use bilge::prelude::1;
use crate::name::Name;

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(8)]
struct MaterialAnimFlags {
	FL_MAT_PLAY: u1,
	FL_MAT_PLAYED: u1,
	FL_MAT_PLAYONCE: u1,
	FL_MAT_NEVERAGAIN: u1,
	FL_MAT_AUTOSTART: u1,
	FLAG_5: u1,
	FLAG_6: u1,
	FLAG_7: u1,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct MaterialAnimBodyV1_381_67_09PC {
	bitmap_name_keyframer: KeyframerHdl,
	scroll_keyframer: KeyframerVec2fLinear,
	scale_keyframer: KeyframerVec2fLinear,
	rotation_keyframer: KeyframerFloatLinearComp,
	diffuse_keyframer: KeyframerVec3fLinear,
	emission_keyframer: KeyframerVec3fLinear,
	alpha_keyframer: KeyframerFloatLinearComp,
	vec4f_keyframer0: KeyframerVec4fLinear,
	params_keyframer: KeyframerVec4fLinear,
	render_flag_keyframer: KeyframerFlag,
	object_flag_keyframer: KeyframerFlag,
	base_material_name: Name,
	duration: f32,
	flags: MaterialAnimFlags,
}

pub type MaterialAnimV1_381_67_09PC = TrivialClass<(), MaterialAnimBodyV1_381_67_09PC>;