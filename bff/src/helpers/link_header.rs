use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::{DynArray, Mat4f, Quat};
use crate::names::Name;
use crate::traits::TryFromGenericSubstitute;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct ResourceObjectLinkHeader {
    #[referenced_names(skip)]
    link_name: Name,
}

// this is just silly. i'm sure there's a better way
impl TryFromGenericSubstitute<Self, Self> for ResourceObjectLinkHeader {
    type Error = crate::error::Error;
    fn try_from_generic_substitute(generic: Self, _: Self) -> Result<Self, Self::Error> {
        Ok(generic)
    }
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
pub struct ObjectDatasFlagsV1_381_67_09PC {
    fl_objectdatas_hide: u1,
    fl_objectdatas_code_control: u1,
    fl_objectdatas_cloned: u1,
    fl_objectdatas_skinned: u1,
    fl_objectdatas_morphed: u1,
    fl_objectdatas_vreflect: u1,
    fl_objectdatas_hide_shadow: u1,
    fl_objectdatas_static_shadow: u1,
    fl_objectdatas_vp0_hide: u1,
    fl_objectdatas_vp1_hide: u1,
    fl_objectdatas_vp2_hide: u1,
    fl_objectdatas_vp3_hide: u1,
    fl_objectdatas_last: u1,
    padding: u19,
}

#[bitsize(32)]
#[derive(
    BinRead, FromBits, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames,
)]
pub struct ObjectFlagsV1_381_67_09PC {
    fl_object_init: u1,
    fl_object_max_bsphere: u1,
    fl_object_skinned: u1,
    fl_object_morphed: u1,
    fl_object_orientedbbox: u1,
    fl_object_no_seaddisplay: u1,
    fl_object_no_seadcollide: u1,
    fl_object_no_display: u1,
    fl_object_transparent: u1,
    fl_object_optimized_vertex: u1,
    fl_object_linear_mapping: u1,
    fl_object_skinned_with_one_bone: u1,
    fl_object_light_baked: u1,
    fl_object_light_baked_with_material: u1,
    fl_object_shadow_receiver: u1,
    fl_object_no_tesselate: u1,
    fl_object_last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u16)]
pub enum ObjectType {
    Points = 0,
    Surface = 1,
    Spline = 2,
    Skin = 3,
    RotShape = 4,
    Lod = 5,
    Mesh = 6,
    Camera = 7,
    SplineZone = 9,
    Occluder = 10,
    CameraZone = 11,
    Light = 12,
    HFog = 13,
    CollisionVol = 14,
    Emiter = 15,
    Omni = 16,
    Graph = 17,
    Particles = 18,
    Flare = 19,
    HField = 20,
    Tree = 21,
    GenWorld = 22,
    Road = 23,
    GenWorldSurface = 24,
    SplineGraph = 25,
    WorldRef = 26,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct ObjectLinkHeaderV1_381_67_09PC {
    #[referenced_names(skip)]
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlagsV1_381_67_09PC,
    r#type: ObjectType,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct ObjectLinkHeaderV1_06_63_02PC {
    #[referenced_names(skip)]
    link_name: Name,
    names: DynArray<Name>,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    pub flags: u32,
    r#type: ObjectType,
}

pub struct ObjectLinkHeaderGeneric {
    pub link_name: Name,
    pub names: DynArray<Name>,
    pub data_name: Name,
    pub rot: Quat,
    pub transform: Mat4f,
    pub radius: f32,
    pub flags: u32,
    pub r#type: ObjectType,
}

impl From<ObjectLinkHeaderV1_381_67_09PC> for ObjectLinkHeaderGeneric {
    fn from(header: ObjectLinkHeaderV1_381_67_09PC) -> Self {
        Self {
            link_name: header.link_name,
            names: vec![].into(),
            data_name: header.data_name,
            rot: header.rot,
            transform: header.transform,
            radius: header.radius,
            flags: header.flags.value,
            r#type: header.r#type,
        }
    }
}

impl From<ObjectLinkHeaderV1_06_63_02PC> for ObjectLinkHeaderGeneric {
    fn from(header: ObjectLinkHeaderV1_06_63_02PC) -> Self {
        Self {
            link_name: header.link_name,
            names: header.names,
            data_name: header.data_name,
            rot: header.rot,
            transform: header.transform,
            radius: header.radius,
            flags: header.flags,
            r#type: header.r#type,
        }
    }
}

impl From<ObjectLinkHeaderGeneric> for ObjectLinkHeaderV1_381_67_09PC {
    fn from(header: ObjectLinkHeaderGeneric) -> Self {
        Self {
            link_name: header.link_name,
            data_name: header.data_name,
            rot: header.rot,
            transform: header.transform,
            radius: header.radius,
            flags: ObjectFlagsV1_381_67_09PC::from(header.flags),
            r#type: header.r#type,
        }
    }
}

impl From<ObjectLinkHeaderGeneric> for ObjectLinkHeaderV1_06_63_02PC {
    fn from(header: ObjectLinkHeaderGeneric) -> Self {
        Self {
            link_name: header.link_name,
            names: header.names,
            data_name: header.data_name,
            rot: header.rot,
            transform: header.transform,
            radius: header.radius,
            flags: header.flags,
            r#type: header.r#type,
        }
    }
}

impl TryFromGenericSubstitute<ObjectLinkHeaderGeneric, Self> for ObjectLinkHeaderV1_06_63_02PC {
    type Error = crate::error::Error;

    fn try_from_generic_substitute(
        generic: ObjectLinkHeaderGeneric,
        _: Self,
    ) -> Result<Self, Self::Error> {
        Ok(generic.into())
    }
}

impl TryFromGenericSubstitute<ObjectLinkHeaderGeneric, Self> for ObjectLinkHeaderV1_381_67_09PC {
    type Error = crate::error::Error;

    fn try_from_generic_substitute(
        generic: ObjectLinkHeaderGeneric,
        _: Self,
    ) -> Result<Self, Self::Error> {
        Ok(generic.into())
    }
}
