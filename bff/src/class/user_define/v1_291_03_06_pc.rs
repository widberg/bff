use std::io::Cursor;

use binrw::BinRead;
use serde::Serialize;

use crate::error::Error;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::strings::PascalString;
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

#[derive(BinRead, Debug, Serialize)]
pub struct UserDefineV1_291_03_06PC {
    data: PascalString,
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
