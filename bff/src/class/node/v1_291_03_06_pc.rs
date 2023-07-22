use std::io::Cursor;

use binrw::BinRead;

use super::Node;
use crate::error::Error;
use crate::math::{Mat4f, Quat, Rect, Sphere, Vec3f, RGBA};
use crate::name::Name;
use crate::object::Object;
use crate::platforms::{platform_to_endian, Platform};
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;
use crate::BffResult;

impl From<NodeV1_291_03_06PC> for Node {
    fn from(node: NodeV1_291_03_06PC) -> Self {
        Node {
            parent_crc32: node.parent_crc32,
            head_child_crc32: node.head_child_crc32,
            prev_node_crc32: node.prev_node_crc32,
            next_node_crc32: node.next_node_crc32,
            object_node_crc32: node.object_node_crc32,
            user_define_crc32: node.user_define_crc32,
            light_data_crc32: Some(node.light_data_crc32),
            bitmap_crc32: node.bitmap_crc32,
            unknown_crc32: node.unknown_crc32,
            inverse_world_transform: node.inverse_world_transform,
            unknown_vec3f1: node.unknown_vec3f1,
            collide_seads_id1: node.collide_seads_id1,
            unknown_vec3f2: node.unknown_vec3f2,
            placeholder_world_matrix_ptr: node.placeholder_world_matrix_ptr,
            display_seads_id1: node.display_seads_id1,
            unknown_matrix: node.unknown_matrix,
            translation: node.translation,
            flags: node.flags,
            rotation: node.rotation,
            scale: node.scale,
            other_scale: node.other_scale,
            one_over_scale: node.one_over_scale,
            unknown_float1: node.unknown_float1,
            color: node.color,
            b_sphere: node.b_sphere,
            display_seads_rect: node.display_seads_rect,
            collide_seads_rect: node.collide_seads_rect,
            world_transform: node.world_transform,
            collide_seads_id2: node.collide_seads_id2,
            display_seads_id2: node.display_seads_id2,
            unknown4: node.unknown4,
            unknown5: node.unknown5,
            unknown6: node.unknown6,
        }
    }
}

#[derive(BinRead, Debug)]
pub struct NodeV1_291_03_06PC {
    parent_crc32: Name,
    head_child_crc32: Name,
    prev_node_crc32: Name,
    next_node_crc32: Name,
    object_node_crc32: Name,
    user_define_crc32: Name,
    light_data_crc32: Name,
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

impl From<Node> for NodeV1_291_03_06PC {
    fn from(node: Node) -> Self {
        NodeV1_291_03_06PC {
            parent_crc32: node.parent_crc32,
            head_child_crc32: node.head_child_crc32,
            prev_node_crc32: node.prev_node_crc32,
            next_node_crc32: node.next_node_crc32,
            object_node_crc32: node.object_node_crc32,
            user_define_crc32: node.user_define_crc32,
            light_data_crc32: match node.light_data_crc32 {
                Some(x) => x,
                None => 0,
            },
            bitmap_crc32: node.bitmap_crc32,
            unknown_crc32: node.unknown_crc32,
            inverse_world_transform: node.inverse_world_transform,
            unknown_vec3f1: node.unknown_vec3f1,
            collide_seads_id1: node.collide_seads_id1,
            unknown_vec3f2: node.unknown_vec3f2,
            placeholder_world_matrix_ptr: node.placeholder_world_matrix_ptr,
            display_seads_id1: node.display_seads_id1,
            unknown_matrix: node.unknown_matrix,
            translation: node.translation,
            flags: node.flags,
            rotation: node.rotation,
            scale: node.scale,
            other_scale: node.other_scale,
            one_over_scale: node.one_over_scale,
            unknown_float1: node.unknown_float1,
            color: node.color,
            b_sphere: node.b_sphere,
            display_seads_rect: node.display_seads_rect,
            collide_seads_rect: node.collide_seads_rect,
            world_transform: node.world_transform,
            collide_seads_id2: node.collide_seads_id2,
            display_seads_id2: node.display_seads_id2,
            unknown4: node.unknown4,
            unknown5: node.unknown5,
            unknown6: node.unknown6,
        }
    }
}

impl TryFromVersionPlatform<&Object> for NodeV1_291_03_06PC {
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> BffResult<NodeV1_291_03_06PC> {
        let mut _header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(NodeV1_291_03_06PC::read_options(
            &mut body_cursor,
            platform_to_endian(platform),
            (),
        )?)
    }
}
