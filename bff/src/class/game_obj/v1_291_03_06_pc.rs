use std::io::Cursor;

use binrw::{binread, BinRead};

use super::GameObj;
use crate::dynarray::DynArray;
use crate::error::Error;
use crate::name::Name;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

#[binread]
#[derive(Debug)]
pub struct GameObjV1_291_03_06PC {
    node_crc32s: DynArray<Name>,
}

impl From<GameObj> for GameObjV1_291_03_06PC {
    fn from(gameobj: GameObj) -> Self {
        GameObjV1_291_03_06PC {
            node_crc32s: gameobj.node_crc32s,
        }
    }
}

impl From<GameObjV1_291_03_06PC> for GameObj {
    fn from(gameobj: GameObjV1_291_03_06PC) -> Self {
        GameObj {
            node_crc32s: gameobj.node_crc32s,
        }
    }
}

impl TryFromVersionPlatform<&Object> for GameObjV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<GameObjV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(GameObjV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
