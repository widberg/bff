use std::io::Cursor;

use binrw::BinRead;
use serde::Serialize;

use super::CollisionVol;
use crate::dynarray::DynArray;
use crate::error::Error;
use crate::math::{Mat4f, Sphere};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::strings::PascalString;
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

impl From<CollisionVolInfo> for super::CollisionVolInfo {
    fn from(collision_vol_info: collision_vol::v1_291_03_06_pc::CollisionVolInfo) -> Self {
        super::CollisionVolInfo {
            local_transform: collision_vol_info.local_transform,
            inv_local_transform: collision_vol_info.inv_local_transform,
        }
    }
}

impl From<CollisionVolV1_291_03_06PC> for CollisionVol {
    fn from(collision_vol: CollisionVolV1_291_03_06PC) -> Self {
        CollisionVol {
            collision_vol_infos: collision_vol.into(),
            in_message_id: collision_vol.in_message_id,
            out_message_id: collision_vol.out_message_id,
            node_param_crc32s: collision_vol.node_param_crc32s,
            float_param_crc32s: collision_vol.float_param_crc32s,
            anim_frame_crc32s: collision_vol.anim_frame_crc32s,
            collision_vol_agent_crc32: collision_vol.collision_vol_agent_crc32,
            anim_start_time: collision_vol.anim_start_time,
        }
    }
}

#[derive(BinRead, Debug)]
struct LinkInfo {
    data_crc32: Name,
    b_sphere_local: Sphere,
    unknown_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    collision_vol_type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    inv_local_transform: Mat4f,
}

#[derive(BinRead, Debug)]
pub struct CollisionVolV1_291_03_06PC {
    collision_vol_infos: DynArray<CollisionVolInfo>,
    in_message_id: u32,
    out_message_id: u32,
    #[br(count = 12)]
    node_param_crc32s: Vec<u32>,
    #[br(count = 12)]
    float_param_crc32s: Vec<f32>,
    anim_frame_crc32s: DynArray<Name>,
    collision_vol_agent_crc32: Name,
    anim_start_time: f32,
}

impl From<CollisionVol> for CollisionVolV1_291_03_06PC {
    fn from(collision_vol: CollisionVol) -> Self {
        CollisionVolV1_291_03_06PC {}
    }
}

impl TryFromVersionPlatform<&Object> for CollisionVolV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<CollisionVolV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(CollisionVolV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
