use bff_derive::GenericClass;

use super::generic::{SoundFlags, SoundGeneric};
use crate::class::trivial_class::TrivialClass;
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::traits::{Export, Import};

#[derive(..BffStruct, GenericClass)]
#[generic(name(SoundHeaderGeneric))]
pub struct SoundHeader {
    #[generic]
    sample_rate: u32,
    #[generic]
    data_size: u32,
    #[generic(no_convert)]
    flags: SoundFlags,
}

#[derive(..BffStruct, GenericClass)]
#[br(import(link_header: &SoundHeader))]
pub struct SoundBodyV1_291_03_06PC {
    #[br(count = link_header.data_size / 2)]
    #[generic]
    data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<SoundHeader, SoundBodyV1_291_03_06PC>;

trivial_class_generic!(SoundV1_291_03_06PC, SoundGeneric);

impl Export for SoundV1_291_03_06PC {}
impl Import for SoundV1_291_03_06PC {}
