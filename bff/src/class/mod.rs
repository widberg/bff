use self::animation::Animation;
use self::binary::Binary;
use self::bitmap::Bitmap;
use self::camera::Camera;
use self::camera_zone::CameraZone;
use self::collision_vol::CollisionVol;
use self::fonts::Fonts;
use self::game_obj::GameObj;
use self::gen_world::GenWorld;
use self::gw_road::GwRoad;
use self::light::Light;
use self::light_data::LightData;
use self::lod::Lod;
use self::lod_data::LodData;
use self::material::Material;
use self::material_anim::MaterialAnim;
use self::material_obj::MaterialObj;
use self::mesh::Mesh;
use self::mesh_data::MeshData;
use self::node::Node;
use self::omni::Omni;
use self::particles::Particles;
use self::particles_data::ParticlesData;
use self::rot_shape::RotShape;
use self::rot_shape_data::RotShapeData;
use self::rtc::Rtc;
use self::skel::Skel;
use self::skin::Skin;
use self::sound::Sound;
use self::spline::Spline;
use self::spline_graph::SplineGraph;
use self::surface::Surface;
use self::surface_datas::SurfaceDatas;
use self::user_define::UserDefine;
use self::warp::Warp;
use self::world::World;
use self::world_ref::WorldRef;
use crate::macros::classes::classes;

pub mod animation;
pub mod binary;
pub mod bitmap;
pub mod camera;
pub mod camera_zone;
pub mod collision_vol;
pub mod fonts;
pub mod game_obj;
pub mod gen_world;
pub mod gw_road;
pub mod light;
pub mod light_data;
pub mod lod;
pub mod lod_data;
pub mod material;
pub mod material_anim;
pub mod material_obj;
pub mod mesh;
pub mod mesh_data;
pub mod node;
pub mod omni;
pub mod particles;
pub mod particles_data;
pub mod rot_shape;
pub mod rot_shape_data;
pub mod rtc;
pub mod skel;
pub mod skin;
pub mod sound;
pub mod spline;
pub mod spline_graph;
pub mod surface;
pub mod surface_datas;
pub mod trivial_class;
pub mod user_define;
pub mod warp;
pub mod world;
pub mod world_ref;

classes! {
    Animation,
    Binary,
    Bitmap,
    Camera,
    CameraZone,
    CollisionVol,
    Fonts,
    GameObj,
    GenWorld,
    GwRoad,
    Light,
    LightData,
    Lod,
    LodData,
    Material,
    MaterialAnim,
    MaterialObj,
    Mesh,
    MeshData,
    Node,
    Omni,
    Particles,
    ParticlesData,
    RotShape,
    RotShapeData,
    Rtc,
    Skel,
    Skin,
    Sound,
    Spline,
    SplineGraph,
    Surface,
    SurfaceDatas,
    UserDefine,
    Warp,
    World,
    WorldRef,
}
