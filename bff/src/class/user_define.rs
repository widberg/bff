use std::io::Cursor;

use ascii::AsciiString;
use binrw::BinRead;
use serde::Serialize;

use crate::error::{Error, UnimplementedClassError};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::strings::PascalString;
use crate::traits::{ShadowClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::{crc32, BffResult};

#[derive(Serialize, Debug)]
pub struct UserDefine {
    data: AsciiString,
}

impl ShadowClass for UserDefine {
    const NAME: Name = crc32::asobo(b"UserDefine_Z");
}

impl TryFromVersionPlatform<&Object> for UserDefine {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        version: Version,
        platform: Platform,
    ) -> BffResult<UserDefine> {
        match (version, platform) {
            (Version::V1_291_03_06, Platform::PC) => {
                let user_define: UserDefineV1_291_03_06PC =
                    UserDefineV1_291_03_06PC::try_from_version_platform(object, version, platform)?;
                Ok(user_define.into())
            }
            _ => Err(
                UnimplementedClassError::new(object.name(), Self::NAME, version, platform).into(),
            ),
        }
    }
}

impl From<UserDefineV1_291_03_06PC> for UserDefine {
    fn from(user_define: UserDefineV1_291_03_06PC) -> Self {
        UserDefine {
            data: (*user_define.data).clone(),
        }
    }
}

#[derive(BinRead, Debug)]
pub struct UserDefineV1_291_03_06PC {
    data: PascalString,
}

impl From<UserDefine> for UserDefineV1_291_03_06PC {
    fn from(user_define: UserDefine) -> Self {
        UserDefineV1_291_03_06PC {
            data: PascalString(user_define.data),
        }
    }
}

impl TryFromVersionPlatform<&Object> for UserDefineV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<UserDefineV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(UserDefineV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
