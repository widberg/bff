use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ResourceObjectLinkHeader;
use crate::map::BffMap;
use crate::math::Vec2f;
use crate::name::Name;

type CharacterID = u32;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct Character {
    material_index: u32,
    descent: f32,
    top_left_corner: Vec2f,
    bottom_right_corner: Vec2f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct FontsBodyV1_381_67_09PC {
    characters: BffMap<CharacterID, Character>,
    material_names: DynArray<Name>,
}

pub type FontsV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, FontsBodyV1_381_67_09PC>;
