use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct MaterialObjBodyV1_381_67_09PC {
	entries: BffMap<Name, DynArray<Name>>,
}

pub type MaterialObjV1_381_67_09PC = TrivialClass<(), MaterialObjBodyV1_381_67_09PC>;