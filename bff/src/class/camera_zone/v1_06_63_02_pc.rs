use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_06_63_02PC, RGBA, Vec2f, Vec3f, Vec4f};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct RangeSizeOffset {
    size: u16,
    offset: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SplineZoneSead {
    p_min: Vec2f,
    p_max: Vec2f,
    inv_diag: Vec2f,
    max_zone_index: u32,
    size_x: u32,
    size_y: u32,
    #[br(count = size_x * size_y)]
    grid_da: Vec<RangeSizeOffset>,
    zone_indices: DynArray<u16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Spline {
    pt_0_id: u16,
    pt_1_id: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SplineZone {
    y: f32,
    spline_ids_ref: RangeSizeOffset,
    unknown0: u16,
    point_flag: u16,
    unknown1: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SplineZoneZ {
    unknown: Vec4f,
    points: DynArray<Vec3f>,
    splines: DynArray<Spline>,
    spline_zones: DynArray<SplineZone>,
    spline_ids: DynArray<u16>,
    unknowns: DynArray<Vec3f>,
    spline_zone_sead: SplineZoneSead,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Trigger {
    rot: f32,
    fov: f32,
    height: f32,
    dist: f32,
    fog: f32,
    far: f32,
    smooth: f32,
    color: RGBA,
    flag: u16,
    spline_id: u16,
    point_id: u16,
    at_point_id: u16,
    spline_length: f32,
    unknown: Vec3f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ZoneTriggers {
    trigger_ids_ref: RangeSizeOffset,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct CameraZoneBodyV1_06_63_02PC {
    spline_zone: SplineZoneZ,
    triggers: DynArray<Trigger>,
    zone_triggers: DynArray<ZoneTriggers>,
    trigger_ids: DynArray<u16>,
}

pub type CameraZoneV1_06_63_02PC =
    TrivialClass<ObjectLinkHeaderV1_06_63_02PC, CameraZoneBodyV1_06_63_02PC>;

impl Export for CameraZoneV1_06_63_02PC {}
impl Import for CameraZoneV1_06_63_02PC {}
