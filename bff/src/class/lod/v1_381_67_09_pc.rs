use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::map::BffMap;
use crate::math::{DynBox, DynSphere};
use crate::names::Name;
use crate::option::BffOption;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LodBodyV1_381_67_09PC {
    collision_spheres: DynArray<DynSphere>,
    collision_boxes: DynArray<DynBox>,
    fade: FadeDistances,
    skin_or_mesh_or_particles_names: DynArray<Name>,
    zero: u32,
    animation_entries: BffOption<BffMap<u32, Name>, u32>,
    sound_entries: BffOption<BffMap<u32, Name>, u32>,
    user_define_name: Name,
}

pub type LodV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, LodBodyV1_381_67_09PC>;
