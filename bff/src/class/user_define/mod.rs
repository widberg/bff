use ascii::AsciiString;
use serde::Serialize;

use crate::error::{Error, UnimplementedClassError};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{ShadowClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::{crc32, BffResult};

mod v1_291_03_06_pc;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;

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
