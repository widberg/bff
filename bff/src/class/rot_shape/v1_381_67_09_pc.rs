use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;

#[bitsize(16)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum BillboardMode {
	Y_BILLBOARD = 0,
	COMPLETE_BILLBOARD = 1,
}

#[derive(BinRead, Debug, Serialize)]
pub struct Points {
	//FIXME: inherits Object_Z
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &Points))]
pub struct RotShapeBodyV1_381_67_09PC {
	origins: DynArray<Vec3f>,
	zero: f32,
	material_anim_names_indices: DynArray<u32>,
	sizes: DynArray<Vec3f>,
	texcoords: DynArray<Vec2f>,
	material_anim_names: DynArray<Name>,
	scale: f32,
	billboard_mode: BillboardMode,
}

pub type RotShapeV1_381_67_09PC = TrivialClass<(), RotShapeBodyV1_381_67_09PC>;