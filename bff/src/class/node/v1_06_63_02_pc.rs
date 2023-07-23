use std::io::Cursor;

use binrw::BinRead;
use serde::Serialize;

use crate::error::Error;
use crate::math::{Mat4f, Quat, Rect, Sphere, Vec3f, RGBA};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

#[derive(BinRead, Debug, Serialize)]
pub struct NodeV1_06_63_02PC {
    parent_crc32: Name,
    head_child_crc32: Name,
    prev_node_crc32: Name,
    next_node_crc32: Name,
    object_node_crc32: Name,
    user_define_crc32: Name,
    bitmap_crc32: Name,
    unknown_crc32: Name,
    inverse_world_transform: Mat4f,
    unknown_vec3f1: Vec3f,
    collide_seads_id1: Name,
    unknown_vec3f2: Vec3f,
    placeholder_world_matrix_ptr: u32,
    display_seads_id1: Name,
    unknown_matrix: Mat4f,
    translation: Vec3f,
    flags: u32,
    rotation: Quat,
    scale: f32,
    other_scale: f32,
    one_over_scale: f32,
    unknown_float1: f32,
    color: RGBA,
    b_sphere: Sphere,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    world_transform: Mat4f,
    collide_seads_id2: Name,
    display_seads_id2: Name,
    unknown4: u16,
    unknown5: u16,
    unknown6: u16,
}

impl TryFromVersionPlatform<&Object> for NodeV1_06_63_02PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<NodeV1_06_63_02PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(NodeV1_06_63_02PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
