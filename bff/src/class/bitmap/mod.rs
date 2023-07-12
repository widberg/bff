use serde::Serialize;

use crate::error::{Error, UnimplementedClassError};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{ShadowClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::{crc32, BffResult};

mod v1_291_03_06_pc;
use v1_291_03_06_pc::BitmapV1_291_03_06PC;

#[derive(Serialize, Debug)]
pub struct Bitmap {
    size: (u32, u32),
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
    data: Vec<u8>,
}

impl ShadowClass for Bitmap {
    const NAME: Name = crc32::asobo(b"Bitmap_Z");
}

impl TryFromVersionPlatform<&Object> for Bitmap {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        version: Version,
        platform: Platform,
    ) -> BffResult<Bitmap> {
        match (version, platform) {
            (Version::V1_291_03_06, Platform::PC) => {
                let bitmap: BitmapV1_291_03_06PC =
                    BitmapV1_291_03_06PC::try_from_version_platform(object, version, platform)?;
                Ok(bitmap.into())
            }
            _ => Err(
                UnimplementedClassError::new(object.name(), Self::NAME, version, platform).into(),
            ),
        }
    }
}
