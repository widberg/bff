use serde::Serialize;

use crate::dynarray::DynArray;
use crate::error::{Error, UnimplementedClassError};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{ShadowClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::{crc32, BffResult};

mod v1_291_03_06_pc;
use v1_291_03_06_pc::GameObjV1_291_03_06PC;

#[derive(Serialize, Debug)]
pub struct GameObj {
    node_crc32s: DynArray<Name>,
}

impl ShadowClass for GameObj {
    const NAME: Name = crc32::asobo(b"GameObj_Z");
}

impl TryFromVersionPlatform<&Object> for GameObj {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        version: Version,
        platform: Platform,
    ) -> BffResult<GameObj> {
        match (version, platform) {
            (Version::V1_291_03_06, Platform::PC) => {
                let gameobj: GameObjV1_291_03_06PC =
                    GameObjV1_291_03_06PC::try_from_version_platform(object, version, platform)?;
                Ok(gameobj.into())
            }
            (Version::V1_291_03_01, Platform::PSP) => {
                let gameobj: GameObjV1_291_03_06PC =
                    GameObjV1_291_03_06PC::try_from_version_platform(object, version, platform)?;
                Ok(gameobj.into())
            }
            _ => Err(
                UnimplementedClassError::new(object.name(), Self::NAME, version, platform).into(),
            ),
        }
    }
}
