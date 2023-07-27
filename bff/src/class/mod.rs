use derive_more::{From, IsVariant};
use serde::Serialize;

use self::animation::Animation;
use self::bitmap::Bitmap;
use self::collision_vol::CollisionVol;
use self::game_obj::GameObj;
use self::light::Light;
use self::light_data::LightData;
use self::lod::Lod;
use self::lod_data::LodData;
use self::material::Material;
use self::mesh::Mesh;
use self::mesh_data::MeshData;
use self::node::Node;
use self::rot_shape::RotShape;
use self::skel::Skel;
use self::skin::Skin;
use self::sound::Sound;
use self::surface::Surface;
use self::user_define::UserDefine;
use self::world::World;
use crate::error::{Error, UnimplementedClassError};
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{NamedClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::BffResult;

pub mod animation;
pub mod bitmap;
pub mod collision_vol;
pub mod game_obj;
pub mod light;
pub mod light_data;
pub mod lod;
pub mod lod_data;
pub mod material;
pub mod mesh;
pub mod mesh_data;
pub mod node;
pub mod rot_shape;
pub mod skel;
pub mod skin;
pub mod sound;
pub mod surface;
pub mod trivial_class;
pub mod user_define;
pub mod world;

macro_rules! classes_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Serialize, Debug, From, IsVariant)]
        #[serde(untagged)]
        pub enum Class {
            $($i(Box<$i>),)*
        }
    };
}

macro_rules! objects_to_classes {
    ($($i:ident),* $(,)?) => {
        impl TryFromVersionPlatform<&Object> for Class {
            type Error = Error;

            fn try_from_version_platform(object: &Object, version: Version, platform: Platform) -> BffResult<Class> {
                match object.class_name() {
                    $(<$i as NamedClass>::NAME => Ok(Box::new(<$i as TryFromVersionPlatform<&Object>>::try_from_version_platform(object, version, platform)?).into()),)*
                    _ => Err(UnimplementedClassError::new(object.name(), object.class_name(), version, platform).into())
                }
            }
        }
    };
}

macro_rules! classes {
    ($($i:ident),* $(,)?) => {
        classes_enum!($($i),*);
        objects_to_classes!($($i),*);
    };
}

classes! {
    Animation,
    Bitmap,
    CollisionVol,
    GameObj,
    Light,
    LightData,
    Lod,
    LodData,
    Material,
    Mesh,
    MeshData,
    Node,
    RotShape,
    Skel,
    Skin,
    Sound,
    Surface,
    UserDefine,
    World,
}
