use bff_derive::bff_class;

pub mod generic;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::SoundV1_291_03_06PC;
use v1_381_67_09_pc::SoundV1_381_67_09PC;

bff_class!(#![generic] Sound {
    (Asobo(1, 6..=291, _, _), _) => SoundV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => SoundV1_381_67_09PC,
});
