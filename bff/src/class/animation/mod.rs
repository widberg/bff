use crate::macros::bff_class::bff_class;

pub mod generic;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::AnimationV1_291_03_06PC;
use v1_381_67_09_pc::AnimationV1_381_67_09PC;

bff_class!(#![generic] Animation {
    (Asobo(1, 6..=291, _, _), PC) => AnimationV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => AnimationV1_381_67_09PC,
});
