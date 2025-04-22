use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::{DynArray, Mat4f, Quat};
use crate::names::Name;
use crate::traits::TryFromGenericSubstitute;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct ResourceLinkHeader {
    #[referenced_names(skip)]
    link_name: Name,
}

// this is just silly. i'm sure there's a better way
impl TryFromGenericSubstitute<Self, Self> for ResourceLinkHeader {
    type Error = crate::error::Error;
    fn try_from_generic_substitute(generic: Self, _: Self) -> Result<Self, Self::Error> {
        Ok(generic)
    }
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
pub struct ResourceDatasFlagsV1_381_67_09PC {
    hide: u1,
    code_control: u1,
    cloned: u1,
    skinned: u1,
    morphed: u1,
    vreflect: u1,
    hide_shadow: u1,
    static_shadow: u1,
    vp0_hide: u1,
    vp1_hide: u1,
    vp2_hide: u1,
    vp3_hide: u1,
    last: u1,
    padding: u19,
}

#[bitsize(32)]
#[derive(
    BinRead, FromBits, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames,
)]
pub struct ResourceFlagsV1_381_67_09PC {
    init: u1,
    max_bsphere: u1,
    skinned: u1,
    morphed: u1,
    orientedbbox: u1,
    no_seaddisplay: u1,
    no_seadcollide: u1,
    no_display: u1,
    transparent: u1,
    optimized_vertex: u1,
    linear_mapping: u1,
    skinned_with_one_bone: u1,
    light_baked: u1,
    light_baked_with_material: u1,
    shadow_receiver: u1,
    no_tesselate: u1,
    last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u16)]
pub enum ResourceType {
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
pub struct ResourceLinkHeaderV1_381_67_09PC {
    #[referenced_names(skip)]
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ResourceFlagsV1_381_67_09PC,
    r#type: ResourceType,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct ResourceLinkHeaderV1_06_63_02PC {
    #[referenced_names(skip)]
    link_name: Name,
    names: DynArray<Name>,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    pub flags: u32,
    r#type: ResourceType,
}

pub struct ResourceLinkHeaderGeneric {
    pub link_name: Name,
    pub names: DynArray<Name>,
    pub data_name: Name,
    pub rot: Quat,
    pub transform: Mat4f,
    pub radius: f32,
    pub flags: u32,
    pub r#type: ResourceType,
}

impl From<ResourceLinkHeaderV1_381_67_09PC> for ResourceLinkHeaderGeneric {
    fn from(header: ResourceLinkHeaderV1_381_67_09PC) -> Self {
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

impl From<ResourceLinkHeaderV1_06_63_02PC> for ResourceLinkHeaderGeneric {
    fn from(header: ResourceLinkHeaderV1_06_63_02PC) -> Self {
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

impl From<ResourceLinkHeaderGeneric> for ResourceLinkHeaderV1_381_67_09PC {
    fn from(header: ResourceLinkHeaderGeneric) -> Self {
        Self {
            link_name: header.link_name,
            data_name: header.data_name,
            rot: header.rot,
            transform: header.transform,
            radius: header.radius,
            flags: ResourceFlagsV1_381_67_09PC::from(header.flags),
            r#type: header.r#type,
        }
    }
}

impl From<ResourceLinkHeaderGeneric> for ResourceLinkHeaderV1_06_63_02PC {
    fn from(header: ResourceLinkHeaderGeneric) -> Self {
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

impl TryFromGenericSubstitute<ResourceLinkHeaderGeneric, Self> for ResourceLinkHeaderV1_06_63_02PC {
    type Error = crate::error::Error;

    fn try_from_generic_substitute(
        generic: ResourceLinkHeaderGeneric,
        _: Self,
    ) -> Result<Self, Self::Error> {
        Ok(generic.into())
    }
}

impl TryFromGenericSubstitute<ResourceLinkHeaderGeneric, Self>
    for ResourceLinkHeaderV1_381_67_09PC
{
    type Error = crate::error::Error;

    fn try_from_generic_substitute(
        generic: ResourceLinkHeaderGeneric,
        _: Self,
    ) -> Result<Self, Self::Error> {
        Ok(generic.into())
    }
}
