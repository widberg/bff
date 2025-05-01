use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::SoundFlags;
use crate::traits::{Export, Import};

#[derive(
    Debug, Clone, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass,
)]
#[generic(name(SoundHeaderGeneric))]
pub struct SoundHeader {
    #[generic]
    sample_rate: u32,
    #[generic]
    data_size: u32,
    #[generic(no_convert)]
    flags: SoundFlags,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(link_header: &SoundHeader))]
pub struct SoundBodyV1_291_03_06PC {
    #[br(count = link_header.data_size / 2)]
    #[serde(skip_serializing)]
    #[generic]
    data: Vec<i16>,
}

trivial_class!(
    SoundV1_291_03_06PC(SoundHeader, SoundBodyV1_291_03_06PC),
    SoundGeneric
);

impl Export for SoundV1_291_03_06PC {}
impl Import for SoundV1_291_03_06PC {}
