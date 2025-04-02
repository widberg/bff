use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    DynArray,
    KeyframerFloatLinear,
    KeyframerVec2fLinear,
    KeyframerVec3fLinear,
    KeyframerVec4fLinear,
    Mat4f,
    ObjectLinkHeaderV1_381_67_09PC,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
struct ParticlesEmitterFlags {
    fl_particles_loop: u1,
    fl_particles_lock_h: u1,
    fl_particles_lock_v: u1,
    fl_particles_use_total: u1,
    fl_particles_noemit: u1,
    fl_particles_oriented: u1,
    fl_particles_noderel: u1,
    fl_particles_boundary_only: u1,
    fl_particles_flip_h: u1,
    fl_particles_flip_v: u1,
    fl_particles_sizex_only: u1,
    fl_particles_light: u1,
    fl_particles_screen: u1,
    fl_particles_screenxy: u1,
    fl_particles_accurate: u1,
    fl_particles_last: u1,
    padding: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ParticlesEmitter {
    max_quantity: u16,
    p_cloud_type: u16,
    p_cloud_size: Vec3f,
    p_cloud_offset: Vec3f,
    off_axis: f32,
    off_axis_variation: f32,
    off_plane: f32,
    off_plane_variation: f32,
    velocity: f32,
    velocity_variation: f32,
    emitter_speed: f32,
    emitter_speed_variation: f32,
    loop_period: f32,
    life: f32,
    life_variation: f32,
    flags: ParticlesEmitterFlags,
    unknown60: KeyframerVec2fLinear,
    unknown61: KeyframerVec4fLinear,
    unknown62: KeyframerVec4fLinear,
    unknown63: KeyframerFloatLinear,
    unknown64: KeyframerVec3fLinear,
    unknown65: KeyframerVec3fLinear,
    unknown66: KeyframerFloatLinear,
    material_anim_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct ParticlesBodyV1_381_67_09PC {
    particles_emitters: DynArray<ParticlesEmitter>,
    mats: DynArray<Mat4f>,
    unknown2: f32,
    unknown3: u16,
}

pub type ParticlesV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, ParticlesBodyV1_381_67_09PC>;

impl Export for ParticlesV1_381_67_09PC {}
impl Import for ParticlesV1_381_67_09PC {}
