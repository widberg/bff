use crate::class::trivial_class::TrivialClass;
use bilge::prelude::1;
use bilge::prelude::23;
use bilge::prelude::5;
use bilge::prelude::26;
use crate::math::RGB;
use crate::math::Vec2f;
use crate::name::Name;

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(32)]
struct MaterialEnabledBitmaps {
	DIFFUSE: u1,
	UNUSED0: u1,
	SPECULAR: u1,
	ADD_NORMAL_LOCAL: u1,
	OCCLUSION: u1,
	NORMAL: u1,
	DIRT: u1,
	NORMAL_LOCAL: u1,
	UNUSED1: u1,
	#[br(temp)]	padding: u23,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(32)]
struct MaterialRdrFlags {
	#[br(temp)]	padding: u5,
	TRANSPARENCY: u1,
	#[br(temp)]	padding: u26,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct MaterialBodyV1_381_67_09PC {
	diffuse: RGB,
	opacity: f32,
	emission: RGB,
	unknown0: i32,
	t_matrix_top_left: Vec2f,
	some_number: i32,
	flags1: u32,
	t_matrix_bottom_right: Vec2f,
	enabled_bitmaps: MaterialEnabledBitmaps,
	rdr_flag: MaterialRdrFlags,
	t_matrix_offset: Vec2f,
	t_matrix_scale: f32,
	t_rotation: f32,
	specular: RGB,
	alpha_ref: f32,
	env_map_factor: f32,
	spec_map_factor: f32,
	bump_map_factor: f32,
	param4: f32,
	t_translation: Vec2f,
	t_scale: Vec2f,
	something_bitmap_related: Vec2f,
	s_diffuse_bitmap_name: Name,
	unused_bitmap_name0: Name,
	s_specular_bitmap_name: Name,
	s_add_normal_local_bitmap_name1: Name,
	s_occlusion_bitmap_name: Name,
	s_normal_bitmap_name: Name,
	s_dirt_bitmap_name: Name,
	s_normal_local_bitmap_name: Name,
	unused_bitmap_name1: Name,
}

pub type MaterialV1_381_67_09PC = TrivialClass<(), MaterialBodyV1_381_67_09PC>;