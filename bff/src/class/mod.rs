use derive_more::From;
use serde::Serialize;

use self::bitmap::Bitmap;
// use self::collision_vol::CollisionVol;
use self::game_obj::GameObj;
// use self::mesh::Mesh;
use self::node::Node;
use self::sound::Sound;
use self::user_define::UserDefine;
use crate::error::{Error, UnimplementedClassError};
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{NamedClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::BffResult;

pub mod bitmap;
// pub mod collision_vol;
pub mod game_obj;
// pub mod mesh;
pub mod node;
pub mod sound;
pub mod user_define;

macro_rules! classes_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Serialize, Debug, From)]
        #[serde(untagged)]
        pub enum Class {
            $($i($i),)*
        }
    };
}

macro_rules! objects_to_classes {
    ($($i:ident),* $(,)?) => {
        impl TryFromVersionPlatform<&Object> for Class {
            type Error = Error;

            fn try_from_version_platform(object: &Object, version: Version, platform: Platform) -> BffResult<Class> {
                match object.class_name() {
                    $(<$i as NamedClass>::NAME => Ok(<$i as TryFromVersionPlatform<&Object>>::try_from_version_platform(object, version, platform)?.into()),)*
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
    Bitmap,
    // CollisionVol,
    GameObj,
    // Mesh,
    Node,
    Sound,
    UserDefine,
}
