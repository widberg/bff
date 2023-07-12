use serde::Serialize;

use crate::error::{Error, UnimplementedClassError};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{ShadowClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::{crc32, BffResult};

mod v1_291_03_06_pc;

use v1_291_03_06_pc::SoundV1_291_03_06PC;

#[derive(Serialize, Debug)]
pub struct Sound {
    sample_rate: u32,
    data_size: Option<u32>,
    sound_type: Option<u16>,
    data: Vec<i16>,
}

impl ShadowClass for Sound {
    const NAME: Name = crc32::asobo(b"Sound_Z");
}

impl TryFromVersionPlatform<&Object> for Sound {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        version: Version,
        platform: Platform,
    ) -> BffResult<Sound> {
        match (version, platform) {
            (Version::V1_291_03_06, Platform::PC) => {
                let sound: SoundV1_291_03_06PC =
                    SoundV1_291_03_06PC::try_from_version_platform(object, version, platform)?;
                Ok(sound.into())
            }
            _ => Err(
                UnimplementedClassError::new(object.name(), Self::NAME, version, platform).into(),
            ),
        }
    }
}
