use bilge::prelude::{bitsize, u1, u15, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Vec2f, Vec3f, Vec4f};
use crate::name::Name;

type KeyFloatLinear = KeyLinearTpl<f32>;
type KeyVec2fLinear = KeyLinearTpl<Vec2f>;
type KeyVec3fLinear = KeyLinearTpl<Vec3f>;
type KeyVec4fLinear = KeyLinearTpl<Vec4f>;
type KeyframerFloatLinear = KeyframerTpl<KeyFloatLinear>;
type KeyframerVec2fLinear = KeyframerTpl<KeyVec2fLinear>;
type KeyframerVec3fLinear = KeyframerTpl<KeyVec3fLinear>;
type KeyframerVec4fLinear = KeyframerTpl<KeyVec4fLinear>;

#[derive(BinRead, Debug, Serialize)]
struct KeyLinearTpl<T>
where
    for<'a> T: BinRead + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    time: f32,
    #[br(align_after = 4)]
    value: T,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum KeyframerInterpolationType {
    Smooth = 0x01,
    Linear = 0x02,
    Square = 0x03,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerTpl<TKey>
where
    for<'a> TKey: BinRead + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
{
    interpolation_type: KeyframerInterpolationType,
    keyframes: DynArray<TKey>,
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
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

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
struct ObjectFlags {
    fl_object_init: u1,
    fl_object_max_bsphere: u1,
    fl_object_skinned: u1,
    fl_object_morphed: u1,
    fl_object_orientedbbox: u1,
    fl_object_no_seaddisplay: u1,
    fl_object_no_seadcollide: u1,
    fl_object_no_display: u1,
    fl_object_transparent: u1,
    fl_object_optimized_vertex: u1,
    fl_object_linear_mapping: u1,
    fl_object_skinned_with_one_bone: u1,
    fl_object_light_baked: u1,
    fl_object_light_baked_with_material: u1,
    fl_object_shadow_receiver: u1,
    fl_object_no_tesselate: u1,
    fl_object_last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum ObjectType {
    Points = 0,
    Surface = 1,
    Spline = 2,
    Skin = 3,
    RotShape = 4,
    Lod = 5,
    Mesh = 6,
    Camera = 7,
    SplineZone = 9,
    Occluder = 10,
    CameraZone = 11,
    Light = 12,
    HFog = 13,
    CollisionVol = 14,
    Emiter = 15,
    Omni = 16,
    Graph = 17,
    Particles = 18,
    Flare = 19,
    HField = 20,
    Tree = 21,
    GenWorld = 22,
    Road = 23,
    GenWorldSurface = 24,
    SplineGraph = 25,
    WorldRef = 26,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlags,
    r#type: ObjectType,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct ParticlesBodyV1_381_67_09PC {
    particles_emitters: DynArray<ParticlesEmitter>,
    mats: DynArray<Mat4f>,
    unknown2: f32,
    unknown3: u16,
}

pub type ParticlesV1_381_67_09PC = TrivialClass<LinkHeader, ParticlesBodyV1_381_67_09PC>;
