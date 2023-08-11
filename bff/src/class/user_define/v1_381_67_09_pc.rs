use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct UserDefineBodyV1_381_67_09PC {
	string: PascalString,
}

pub type UserDefineV1_381_67_09PC = TrivialClass<(), UserDefineBodyV1_381_67_09PC>;