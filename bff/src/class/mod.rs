use self::anim_frame::AnimFrame;
use self::animation::Animation;
use self::animation_graph::AnimationGraph;
use self::animation_graph_override::AnimationGraphOverride;
use self::area_light::AreaLight;
use self::binary::Binary;
use self::bitmap::Bitmap;
use self::camera::Camera;
use self::camera_zone::CameraZone;
use self::collision_vol::CollisionVol;
use self::collision_vol_data::CollisionVolData;
use self::conductor::Conductor;
use self::decal::Decal;
use self::dialog_event::DialogEvent;
use self::entity::Entity;
use self::flare::Flare;
use self::flare_data::FlareData;
use self::fog_volume::FogVolume;
use self::fonts::Fonts;
use self::fx_particles::FxParticles;
use self::fx_particles_data::FxParticlesData;
use self::game_obj::GameObj;
use self::gen_world::GenWorld;
use self::graph::Graph;
use self::gw_road::GwRoad;
use self::hfog::HFog;
use self::hfog_data::HFogData;
use self::hull_spline_zone::HullSplineZone;
use self::light::Light;
use self::light_data::LightData;
use self::light_probe_volume::LightProbeVolume;
use self::lod::Lod;
use self::lod_data::LodData;
use self::mass_instancing_volume::MassInstancingVolume;
use self::material::Material;
use self::material_anim::MaterialAnim;
use self::material_collect::MaterialCollect;
use self::material_obj::MaterialObj;
use self::mesh::Mesh;
use self::mesh_data::MeshData;
use self::net_bing_obj::NetBingObj;
use self::node::Node;
use self::occluder::Occluder;
use self::omni::Omni;
use self::omni_data::OmniData;
use self::r#override::Override;
use self::particles::Particles;
use self::particles_data::ParticlesData;
use self::prefab::Prefab;
use self::prefab_ref::PrefabRef;
use self::reflection_probe::ReflectionProbe;
use self::rot_shape::RotShape;
use self::rot_shape_data::RotShapeData;
use self::rtc::Rtc;
use self::shader::Shader;
use self::skel::Skel;
use self::skin::Skin;
use self::skindata::SkinData;
use self::sound::Sound;
use self::sound_event::SoundEvent;
use self::specialeffectnode::SpecialEffectNode;
use self::spline::Spline;
use self::spline_graph::SplineGraph;
use self::spline_zone::SplineZone;
use self::surface::Surface;
use self::surface_datas::SurfaceDatas;
use self::terrain::Terrain;
use self::texture::Texture;
use self::txt::Txt;
use self::user_define::UserDefine;
use self::user_define_script::UserDefineScript;
use self::warp::Warp;
use self::world::World;
use self::world_ref::WorldRef;
use self::xrefnode::XRefNode;
use crate::macros::classes::classes;

pub mod anim_frame;
pub mod animation;
pub mod animation_graph;
pub mod animation_graph_override;
pub mod area_light;
pub mod binary;
pub mod bitmap;
pub mod camera;
pub mod camera_zone;
pub mod collision_vol;
pub mod collision_vol_data;
pub mod conductor;
pub mod decal;
pub mod dialog_event;
pub mod entity;
pub mod flare;
pub mod flare_data;
pub mod fog_volume;
pub mod fonts;
pub mod fx_particles;
pub mod fx_particles_data;
pub mod game_obj;
pub mod gen_world;
pub mod graph;
pub mod gw_road;
pub mod hfog;
pub mod hfog_data;
pub mod hull_spline_zone;
pub mod light;
pub mod light_data;
pub mod light_probe_volume;
pub mod lod;
pub mod lod_data;
pub mod mass_instancing_volume;
pub mod material;
pub mod material_anim;
pub mod material_collect;
pub mod material_obj;
pub mod mesh;
pub mod mesh_data;
pub mod net_bing_obj;
pub mod node;
pub mod occluder;
pub mod omni;
pub mod omni_data;
pub mod r#override;
pub mod particles;
pub mod particles_data;
pub mod prefab;
pub mod prefab_ref;
pub mod reflection_probe;
pub mod rot_shape;
pub mod rot_shape_data;
pub mod rtc;
pub mod shader;
pub mod skel;
pub mod skin;
pub mod skindata;
pub mod sound;
pub mod sound_event;
pub mod specialeffectnode;
pub mod spline;
pub mod spline_graph;
pub mod spline_zone;
pub mod surface;
pub mod surface_datas;
pub mod terrain;
pub mod texture;
pub mod trivial_class;
pub mod txt;
pub mod user_define;
pub mod user_define_script;
pub mod warp;
pub mod world;
pub mod world_ref;
pub mod xrefnode;

classes! {
    Animation,
    AnimationGraph,
    AnimationGraphOverride,
    AnimFrame,
    AreaLight,
    Binary,
    Bitmap,
    Camera,
    CameraZone,
    CollisionVol,
    CollisionVolData,
    Conductor,
    Decal,
    DialogEvent,
    Entity,
    Flare,
    FlareData,
    FogVolume,
    Fonts,
    FxParticles,
    FxParticlesData,
    GameObj,
    GenWorld,
    Graph,
    GwRoad,
    HFog,
    HFogData,
    HullSplineZone,
    Light,
    LightData,
    LightProbeVolume,
    Lod,
    LodData,
    MassInstancingVolume,
    Material,
    MaterialAnim,
    MaterialCollect,
    MaterialObj,
    Mesh,
    MeshData,
    NetBingObj,
    Node,
    Occluder,
    Omni,
    OmniData,
    Override,
    Particles,
    ParticlesData,
    Prefab,
    PrefabRef,
    ReflectionProbe,
    RotShape,
    RotShapeData,
    Rtc,
    Shader,
    Skel,
    Skin,
    SkinData,
    Sound,
    SoundEvent,
    SpecialEffectNode,
    Spline,
    SplineGraph,
    SplineZone,
    Surface,
    SurfaceDatas,
    Terrain,
    Texture,
    Txt,
    UserDefine,
    UserDefineScript,
    Warp,
    World,
    WorldRef,
    XRefNode,
}
