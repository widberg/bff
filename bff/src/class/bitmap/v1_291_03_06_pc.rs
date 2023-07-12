use std::io::Cursor;

use binrw::{binread, BinRead};

use super::Bitmap;
use crate::error::Error;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

#[binread]
#[derive(Debug)]
pub struct BitmapV1_291_03_06PC {
    size: (u32, u32),
    #[br(temp)]
    precalculated_size: u32,
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
    #[br(count = precalculated_size)]
    data: Vec<u8>,
}

impl From<Bitmap> for BitmapV1_291_03_06PC {
    fn from(bitmap: Bitmap) -> Self {
        BitmapV1_291_03_06PC {
            size: bitmap.size,
            flag: bitmap.flag,
            format: bitmap.format,
            mipmap_count: bitmap.mipmap_count,
            unknown: bitmap.unknown,
            data: bitmap.data,
        }
    }
}

impl From<BitmapV1_291_03_06PC> for Bitmap {
    fn from(bitmap: BitmapV1_291_03_06PC) -> Self {
        Bitmap {
            size: bitmap.size,
            flag: bitmap.flag,
            format: bitmap.format,
            mipmap_count: bitmap.mipmap_count,
            unknown: bitmap.unknown,
            data: bitmap.data,
        }
    }
}

impl TryFromVersionPlatform<&Object> for BitmapV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<BitmapV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(BitmapV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
