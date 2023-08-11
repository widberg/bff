use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
struct Prefab {
	string: PascalStringNULL,
	in_world: u32,
	names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct GameObjBodyV1_381_67_09PC {
	prefabs: DynArray<Prefab>,
}

pub type GameObjV1_381_67_09PC = TrivialClass<(), GameObjBodyV1_381_67_09PC>;