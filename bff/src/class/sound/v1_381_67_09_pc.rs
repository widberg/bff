use bff_derive::{GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::generic::{SoundFlags, SoundGeneric};
use crate::class::trivial_class::TrivialClass;
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(
    BinRead,
    Clone,
    Debug,
    Serialize,
    BinWrite,
    Deserialize,
    ReferencedNames,
    GenericClass,
    JsonSchema,
)]
#[generic(name(SoundHeaderGeneric))]
pub struct LinkHeader {
    #[referenced_names(skip)]
    pub link_name: Name,
    #[generic]
    pub sample_rate: u32,
    #[generic]
    pub data_size: u32,
    #[generic(no_convert)]
    pub flags: SoundFlags,
}

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames, GenericClass,
)]
#[br(import(link_header: &LinkHeader))]
pub struct SoundBodyV1_381_67_09PC {
    #[br(count = link_header.data_size / 2)]
    #[generic]
    data: Vec<i16>,
}

pub type SoundV1_381_67_09PC = TrivialClass<LinkHeader, SoundBodyV1_381_67_09PC>;

trivial_class_generic!(SoundV1_381_67_09PC, SoundGeneric);

impl Export for SoundV1_381_67_09PC {}
impl Import for SoundV1_381_67_09PC {}
