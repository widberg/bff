use bff_derive::ReferencedNames;
use binrw::{BinRead, BinResult, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffMap,
    DynArray,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    ResourceObjectLinkHeaderV1_381_67_09PC,
    Vec2f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[binrw::writer(writer, endian)]
fn write_character(c: &char) -> BinResult<()> {
    let mut buf = [0u8; 4];
    let bytes = c.encode_utf8(&mut buf).as_bytes();
    let val = bytes.iter().fold(0u32, |acc, &b| (acc << 8) | b as u32);
    val.write_options(writer, endian, ())
}

#[binrw::parser(reader, endian)]
fn parse_character() -> BinResult<char> {
    let bytes = <u32>::read_options(reader, endian, ())?.to_be_bytes();
    let utf8_bytes = match bytes.iter().position(|&b| b != 0) {
        Some(pos) => &bytes[pos..],
        None => &[0],
    };
    // TODO: Handle errors
    Ok(std::str::from_utf8(utf8_bytes).unwrap().chars().next().unwrap())
}

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, PartialEq, Eq, Hash,
)]
struct CharacterID(
    #[br(parse_with = parse_character)]
    #[bw(write_with = write_character)]
    char,
);

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Character {
    material_index: u32,
    descent: f32,
    top_left_corner: Vec2f,
    bottom_right_corner: Vec2f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct FontsBodyV1_381_67_09PC {
    characters: BffMap<CharacterID, Character>,
    material_names: DynArray<Name>,
}

pub type FontsV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, FontsBodyV1_381_67_09PC>;

impl Export for FontsV1_381_67_09PC {}
impl Import for FontsV1_381_67_09PC {}

// TODO: Shouldn't need to duplicate this just because the link header type is different
#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct FontsBodyV1_06_63_02PC {
    characters: BffMap<CharacterID, Character>,
    material_names: DynArray<Name>,
}

pub type FontsV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, FontsBodyV1_06_63_02PC>;

impl Export for FontsV1_06_63_02PC {}
impl Import for FontsV1_06_63_02PC {}
