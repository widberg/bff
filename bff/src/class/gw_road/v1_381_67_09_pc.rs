use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec2f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Point {
    #[br(big)]
    encoded_vec2f_data0: i32,
    encoded_vec2f_data1: i8,
}

#[derive(BinRead, Debug, Serialize)]
struct Road {
    r#type: u8,
    points: DynArray<Point, u16>,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused5 {
    unused0: u32,
    unused1: u32,
    unused2: u32,
    unused3: u32,
    unused4: u32,
    unused5: u32,
    unused6: u32,
    unused7: u32,
    #[br(count = unused0 & 0xFFFF)]
    unused8s: Vec<u32>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct GwRoadBodyV1_381_67_09PC {
    road_count: u32,
    gen_road_min: Vec2f,
    gen_road_max: Vec2f,
    #[br(count = road_count)]
    roads: Vec<Road>,
    unused5_count: u32,
    unused5_min: Vec2f,
    unused5_max: Vec2f,
    #[br(count = unused5_count)]
    unused5s: Vec<Unused5>,
    gen_world_name: Name,
}

pub type GwRoadV1_381_67_09PC = TrivialClass<ResourceObject, GwRoadBodyV1_381_67_09PC>;
