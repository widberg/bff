use crate::class::trivial_class::TrivialClass;
use crate::name::Name;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
struct SkinSubsection {
	animation_node_names: [Name; 4],
	#[br(count = parent.parent.parent.parent.bone_name_count)]	bone_names: Vec<Name>,
}

#[derive(BinRead, Debug, Serialize)]
struct SkinSection {
	skin_subsections: DynArray<SkinSubsection>,
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
pub struct SkinBodyV1_381_67_09PC {
	mesh_names: DynArray<Name>,
	zeros: [u32; 4],
	one_and_a_half: f32,
	bone_name_count: u32,
	skin_sections: DynArray<SkinSection>,
}

pub type SkinV1_381_67_09PC = TrivialClass<(), SkinBodyV1_381_67_09PC>;