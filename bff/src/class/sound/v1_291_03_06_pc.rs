use std::io::Cursor;

use binrw::BinRead;
use serde::Serialize;

use crate::error::Error;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

#[derive(Debug, BinRead, Serialize)]
pub struct SoundV1_291_03_06PC {
    sample_rate: u32,
    #[brw(if(sample_rate != 0))]
    data_size: Option<u32>,
    #[brw(if(sample_rate != 0))]
    sound_type: Option<u16>,
    #[br(count = data_size.expect("sample rate is 0") / 2)]
    data: Vec<i16>,
}

impl TryFromVersionPlatform<&Object> for SoundV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<SoundV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(SoundV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
