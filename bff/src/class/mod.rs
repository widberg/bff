use self::a_i_object_collection::AIObjectCollection;
use self::a_i_obstacle_collection::AIObstacleCollection;
use self::ambient_lightmap::AmbientLightmap;
use self::anim_frame::AnimFrame;
use self::animation::Animation;
use self::animation_collection::AnimationCollection;
use self::animation_graph::AnimationGraph;
use self::animation_graph_override::AnimationGraphOverride;
use self::animation_stack::AnimationStack;
use self::area_light::AreaLight;
use self::binary::Binary;
use self::bitmap::Bitmap;
use self::camera::Camera;
use self::camera_zone::CameraZone;
use self::collision_vol::CollisionVol;
use self::collision_vol_data::CollisionVolData;
use self::conductor::Conductor;
use self::character_description::CharacterDescription;
use self::data_base_file::DataBaseFile;
use self::data_container::DataContainer;
use self::decal::Decal;
use self::dialog_event::DialogEvent;
use self::embedded_file::EmbeddedFile;
use self::engine_parameters::EngineParameters;
use self::entity::Entity;
use self::entity_data::EntityData;
use self::fence::Fence;
use self::fence_datas::FenceDatas;
use self::flare::Flare;
use self::flare_data::FlareData;
use self::fog_volume::FogVolume;
use self::font3_d::Font3D;
use self::fonts::Fonts;
use self::fx_particles::FxParticles;
use self::fx_particles_data::FxParticlesData;
use self::game_obj::GameObj;
use self::game_parameters::GameParameters;
use self::gen_world::GenWorld;
use self::graph::Graph;
use self::graph_dummy::GraphDummy;
use self::gw_road::GwRoad;
use self::h_fog::HFog;
use self::h_fog_data::HFogData;
use self::hull_spline_zone::HullSplineZone;
use self::in_game_animation_file::InGameAnimationFile;
use self::in_game_file::InGameFile;
use self::lens_flare::LensFlare;
use self::lens_flare_data::LensFlareData;
use self::light::Light;
use self::light_data::LightData;
use self::light_probe_volume::LightProbeVolume;
use self::lip_sync::LipSync;
use self::lod::Lod;
use self::lod_data::LodData;
use self::mass_instancing_volume::MassInstancingVolume;
use self::material::Material;
use self::material_anim::MaterialAnim;
use self::material_collect::MaterialCollect;
use self::material_obj::MaterialObj;
use self::menu_master_menu::MenuMasterMenu;
use self::mesh::Mesh;
use self::mesh_data::MeshData;
use self::navigation_area::NavigationArea;
use self::navigation_spline::NavigationSpline;
use self::net_bing_obj::NetBingObj;
use self::node::Node;
use self::object::Object;
use self::object_datas::ObjectDatas;
use self::occluder::Occluder;
use self::omni::Omni;
use self::omni_data::OmniData;
use self::r#override::Override;
use self::package::Package;
use self::parameter_table_file::ParameterTableFile;
use self::particles::Particles;
use self::particles_data::ParticlesData;
use self::prefab::Prefab;
use self::prefab_ref::PrefabRef;
use self::projector::Projector;
use self::projector_data::ProjectorData;
use self::reflection_probe::ReflectionProbe;
use self::rot_shape::RotShape;
use self::rot_shape_data::RotShapeData;
use self::rtc::Rtc;
use self::shader::Shader;
use self::skel::Skel;
use self::skel_data::SkelData;
use self::skin::Skin;
use self::skin_data::SkinData;
use self::sound::Sound;
use self::sound_ambience::SoundAmbience;
use self::sound_data::SoundData;
use self::sound_event::SoundEvent;
use self::sound_id::SoundId;
use self::sound_node::SoundNode;
use self::special_effect_node::SpecialEffectNode;
use self::spline::Spline;
use self::spline_graph::SplineGraph;
use self::spline_node::SplineNode;
use self::spline_point_node::SplinePointNode;
use self::spline_point_tangent_node::SplinePointTangentNode;
use self::spline_zone::SplineZone;
use self::sub_world::SubWorld;
use self::surface::Surface;
use self::surface_datas::SurfaceDatas;
use self::terrain::Terrain;
use self::texture::Texture;
use self::trigger_node::TriggerNode;
use self::txt::Txt;
use self::u_i3_d_canvas::UI3DCanvas;
use self::u_i_container::UIContainer;
use self::u_i_font::UIFont;
use self::u_i_layout_node::UILayoutNode;
use self::u_i_list_box::UIListBox;
use self::u_i_material::UIMaterial;
use self::u_i_nine_slice::UINineSlice;
use self::u_i_panel::UIPanel;
use self::u_i_text_panel::UITextPanel;
use self::user_define::UserDefine;
use self::user_define_script::UserDefineScript;
use self::warp::Warp;
use self::world::World;
use self::world_ref::WorldRef;
use self::x_ref_node::XRefNode;
use crate::macros::classes::classes;

pub mod a_i_object_collection;
pub mod a_i_obstacle_collection;
pub mod ambient_lightmap;
pub mod anim_frame;
pub mod animation;
pub mod animation_collection;
pub mod animation_graph;
pub mod animation_graph_override;
pub mod animation_stack;
pub mod area_light;
pub mod binary;
pub mod bitmap;
pub mod camera;
pub mod camera_zone;
pub mod collision_vol;
pub mod collision_vol_data;
pub mod conductor;
pub mod character_description;
pub mod data_base_file;
pub mod data_container;
pub mod decal;
pub mod dialog_event;
pub mod embedded_file;
pub mod engine_parameters;
pub mod entity;
pub mod entity_data;
pub mod fence;
pub mod fence_datas;
pub mod flare;
pub mod flare_data;
pub mod fog_volume;
pub mod font3_d;
pub mod fonts;
pub mod fx_particles;
pub mod fx_particles_data;
pub mod game_obj;
pub mod game_parameters;
pub mod gen_world;
pub mod graph;
pub mod graph_dummy;
pub mod gw_road;
pub mod h_fog;
pub mod h_fog_data;
pub mod hull_spline_zone;
pub mod in_game_animation_file;
pub mod in_game_file;
pub mod lens_flare;
pub mod lens_flare_data;
pub mod light;
pub mod light_data;
pub mod light_probe_volume;
pub mod lip_sync;
pub mod lod;
pub mod lod_data;
pub mod mass_instancing_volume;
pub mod material;
pub mod material_anim;
pub mod material_collect;
pub mod material_obj;
pub mod menu_master_menu;
pub mod mesh;
pub mod mesh_data;
pub mod navigation_area;
pub mod navigation_spline;
pub mod net_bing_obj;
pub mod node;
pub mod object;
pub mod object_datas;
pub mod occluder;
pub mod omni;
pub mod omni_data;
pub mod r#override;
pub mod package;
pub mod parameter_table_file;
pub mod particles;
pub mod particles_data;
pub mod prefab;
pub mod prefab_ref;
pub mod projector;
pub mod projector_data;
pub mod reflection_probe;
pub mod rot_shape;
pub mod rot_shape_data;
pub mod rtc;
pub mod shader;
pub mod skel;
pub mod skel_data;
pub mod skin;
pub mod skin_data;
pub mod sound;
pub mod sound_ambience;
pub mod sound_data;
pub mod sound_event;
pub mod sound_id;
pub mod sound_node;
pub mod special_effect_node;
pub mod spline;
pub mod spline_graph;
pub mod spline_node;
pub mod spline_point_node;
pub mod spline_point_tangent_node;
pub mod spline_zone;
pub mod sub_world;
pub mod surface;
pub mod surface_datas;
pub mod terrain;
pub mod texture;
pub mod trigger_node;
pub mod trivial_class;
pub mod txt;
pub mod u_i3_d_canvas;
pub mod u_i_container;
pub mod u_i_font;
pub mod u_i_layout_node;
pub mod u_i_list_box;
pub mod u_i_material;
pub mod u_i_nine_slice;
pub mod u_i_panel;
pub mod u_i_text_panel;
pub mod user_define;
pub mod user_define_script;
pub mod warp;
pub mod world;
pub mod world_ref;
pub mod x_ref_node;

classes! {
    AIObstacleCollection,
    AIObjectCollection,
    AmbientLightmap,
    Animation,
    AnimationCollection,
    AnimationGraph,
    AnimationGraphOverride,
    AnimationStack,
    AnimFrame,
    AreaLight,
    Binary,
    Bitmap,
    Camera,
    CameraZone,
    CharacterDescription,
    CollisionVol,
    CollisionVolData,
    Conductor,
    DataBaseFile,
    DataContainer,
    Decal,
    DialogEvent,
    EmbeddedFile,
    EngineParameters,
    Entity,
    EntityData,
    Fence,
    FenceDatas,
    Flare,
    FlareData,
    FogVolume,
    Font3D,
    Fonts,
    FxParticles,
    FxParticlesData,
    GameObj,
    GameParameters,
    GenWorld,
    Graph,
    GraphDummy,
    GwRoad,
    HFog,
    HFogData,
    HullSplineZone,
    InGameAnimationFile,
    InGameFile,
    LensFlare,
    LensFlareData,
    Light,
    LightData,
    LightProbeVolume,
    LipSync,
    Lod,
    LodData,
    MassInstancingVolume,
    Material,
    MaterialAnim,
    MaterialCollect,
    MaterialObj,
    MenuMasterMenu,
    Mesh,
    MeshData,
    NavigationArea,
    NavigationSpline,
    NetBingObj,
    Node,
    Object,
    ObjectDatas,
    Occluder,
    Omni,
    OmniData,
    Override,
    Package,
    ParameterTableFile,
    Particles,
    ParticlesData,
    Prefab,
    PrefabRef,
    Projector,
    ProjectorData,
    ReflectionProbe,
    RotShape,
    RotShapeData,
    Rtc,
    Shader,
    Skel,
    SkelData,
    Skin,
    SkinData,
    Sound,
    SoundAmbience,
    SoundData,
    SoundEvent,
    SoundId,
    SoundNode,
    SpecialEffectNode,
    Spline,
    SplineGraph,
    SplineNode,
    SplinePointNode,
    SplinePointTangentNode,
    SplineZone,
    SubWorld,
    Surface,
    SurfaceDatas,
    Terrain,
    Texture,
    TriggerNode,
    Txt,
    UI3DCanvas,
    UIContainer,
    UIFont,
    UILayoutNode,
    UIListBox,
    UIMaterial,
    UINineSlice,
    UIPanel,
    UITextPanel,
    UserDefine,
    UserDefineScript,
    Warp,
    World,
    WorldRef,
    XRefNode,
}
