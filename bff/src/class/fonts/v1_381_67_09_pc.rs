use crate::class::trivial_class::TrivialClass;
use crate::math::Vec2f;
use crate::dynarray::DynArray;
type CharacterID = u32;
;
#[derive(BinRead, Debug, Serialize)]
struct Character {
	material_index: u32,
	descent: f32,
	top_left_corner: Vec2f,
	bottom_right_corner: Vec2f,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct FontsBodyV1_381_67_09PC {
	characters: BffMap<CharacterID, Character>,
	material_names: DynArray<Name>,
}

pub type FontsV1_381_67_09PC = TrivialClass<(), FontsBodyV1_381_67_09PC>;