use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
	//FIXME: inherits ResourceObject_Z
	flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct RotShapeDataBodyV1_381_67_09PC {
	zeros: DynArray<u16>,
	#[br(count = zeros.size * 28)]	pad: Vec<u8>,
}

pub type RotShapeDataV1_381_67_09PC = TrivialClass<(), RotShapeDataBodyV1_381_67_09PC>;