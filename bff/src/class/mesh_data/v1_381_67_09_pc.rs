use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
	//FIXME: inherits ResourceObject_Z
	flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct MeshDataBodyV1_381_67_09PC {
	zero0: u32,
	zero1: u32,
	zero2: u32,
	zero3: u32,
}

pub type MeshDataV1_381_67_09PC = TrivialClass<(), MeshDataBodyV1_381_67_09PC>;