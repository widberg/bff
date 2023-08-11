use crate::class::trivial_class::TrivialClass;
use bilge::prelude::1;
use bilge::prelude::16;
use crate::math::Vec3f;
use crate::name::Name;
use crate::dynarray::DynArray;

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(32)]
struct ParticlesEmitter_Flags {
	FL_PARTICLES_LOOP: u1,
	FL_PARTICLES_LOCK_H: u1,
	FL_PARTICLES_LOCK_V: u1,
	FL_PARTICLES_USE_TOTAL: u1,
	FL_PARTICLES_NOEMIT: u1,
	FL_PARTICLES_ORIENTED: u1,
	FL_PARTICLES_NODEREL: u1,
	FL_PARTICLES_BOUNDARY_ONLY: u1,
	FL_PARTICLES_FLIP_H: u1,
	FL_PARTICLES_FLIP_V: u1,
	FL_PARTICLES_SIZEX_ONLY: u1,
	FL_PARTICLES_LIGHT: u1,
	FL_PARTICLES_SCREEN: u1,
	FL_PARTICLES_SCREENXY: u1,
	FL_PARTICLES_ACCURATE: u1,
	FL_PARTICLES_LAST: u1,
	#[br(temp)]	padding: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct ParticlesEmitter {
	max_quantity: u16,
	p_cloud_type: u16,
	p_cloud_size: Vec3f,
	p_cloud_offset: Vec3f,
	off_axis: f32,
	off_axis_variation: f32,
	off_plane: f32,
	off_plane_variation: f32,
	velocity: f32,
	velocity_variation: f32,
	emitter_speed: f32,
	emitter_speed_variation: f32,
	loop_period: f32,
	life: f32,
	life_variation: f32,
	flags: ParticlesEmitter_Flags,
	unknown60: KeyframerVec2fLinear,
	unknown61: KeyframerVec4fLinear,
	unknown62: KeyframerVec4fLinear,
	unknown63: KeyframerFloatLinear,
	unknown64: KeyframerVec3fLinear,
	unknown65: KeyframerVec3fLinear,
	unknown66: KeyframerFloatLinear,
	material_anim_name: Name,
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
pub struct ParticlesBodyV1_381_67_09PC {
	particles_emitters: DynArray<ParticlesEmitter>,
	mats: DynArray<Mat4f>,
	unknown2: f32,
	unknown3: u16,
}

pub type ParticlesV1_381_67_09PC = TrivialClass<(), ParticlesBodyV1_381_67_09PC>;