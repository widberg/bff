use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::math::{Mat3f, RGB, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &()))]
pub struct MaterialBodyV1_291_03_06PC {
    diffuse_color: RGBA,
    emissive_color: RGB,
    cdcdcdcd: u32,
    uv_transform_matrix: Mat3f,
    unknown1s: [f32; 8],
    unknown2s: [u32; 3],
    diffuse_translation: [f32; 2],
    diffuse_scale: [f32; 2],
    diffuse_rotation: f32,
    flags: [u32; 3],
    texture_flag: u8,
    #[br(count = match texture_flag {
        1 => 8,
        3 => 2,
        _ => 4,
    })]
    textures: Vec<u32>,
}

pub type MaterialV1_291_03_06PC = TrivialClass<(), MaterialBodyV1_291_03_06PC>;
