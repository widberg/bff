use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::SoundFlags;
use crate::class::trivial_class::TrivialClass;
use crate::traits::{Export, Import};

#[derive(
    Debug, Clone, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass,
)]
pub struct SoundHeader {
    #[generic]
    sample_rate: u32,
    #[generic]
    data_size: u32,
    #[generic(no_convert)]
    flags: SoundFlags,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    header: SoundHeader,
    #[br(count = header.data_size / 2)]
    #[serde(skip_serializing)]
    #[generic]
    data: Vec<i16>,
}

trivial_class!(
    SoundV1_291_03_06PC((), SoundBodyV1_291_03_06PC),
    SoundGeneric
);

impl Export for SoundV1_291_03_06PC {}
impl Import for SoundV1_291_03_06PC {}
