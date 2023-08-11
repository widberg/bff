use crate::class::trivial_class::TrivialClass;
use crate::math::Vec3f;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
	//FIXME: inherits ResourceObject_Z
	flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct ParticlesDataBodyV1_381_67_09PC {
	position: Vec3f,
	fade: FadeDistances,
	shorts: DynArray<u16>,
	zero: u32,
}

pub type ParticlesDataV1_381_67_09PC = TrivialClass<(), ParticlesDataBodyV1_381_67_09PC>;