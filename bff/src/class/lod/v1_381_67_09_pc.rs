use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
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
pub struct LodBodyV1_381_67_09PC {
	collision_spheres: DynArray<DynSphere>,
	collision_boxes: DynArray<DynBox>,
	fade: FadeDistances,
	skin_or_mesh_or_particles_names: DynArray<Name>,
	zero: u32,
	animation_entries: OptionalU32<Map<u32, Name>>,
	sound_entries: OptionalU32<Map<u32, Name>>,
	user_define_name: Name,
}

pub type LodV1_381_67_09PC = TrivialClass<(), LodBodyV1_381_67_09PC>;