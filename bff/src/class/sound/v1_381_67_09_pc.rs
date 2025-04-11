use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::SoundFlags;
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(
    BinRead, Clone, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass,
)]
#[generic(name(SoundHeaderGeneric))]
pub struct LinkHeader {
    #[referenced_names(skip)]
    link_name: Name,
    #[generic]
    sample_rate: u32,
    #[generic]
    data_size: u32,
    #[generic(no_convert)]
    flags: SoundFlags,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(link_header: &LinkHeader))]
pub struct SoundBodyV1_381_67_09PC {
    #[br(count = link_header.data_size / 2)]
    #[generic]
    data: Vec<i16>,
}

trivial_class!(
    SoundV1_381_67_09PC(LinkHeader, SoundBodyV1_381_67_09PC),
    SoundGeneric
);

impl Export for SoundV1_381_67_09PC {}
impl Import for SoundV1_381_67_09PC {}
