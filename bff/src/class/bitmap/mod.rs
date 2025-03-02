use bff_derive::bff_class;

pub mod generic;
pub mod v1_06_63_02_pc;
pub mod v1_291_03_06_pc;
pub mod v1_381_67_09_pc;

use generic::BitmapGeneric;
use v1_06_63_02_pc::BitmapV1_06_63_02PC;
use v1_291_03_06_pc::BitmapV1_291_03_06PC;
use v1_381_67_09_pc::BitmapV1_381_67_09PC;

use crate::traits::{TryFromGenericSubstitute, TryIntoSpecific};

bff_class!(#![generic] Bitmap {
    (Asobo(1, 6, 63, 2), PC) => BitmapV1_06_63_02PC,
    (Asobo(1, 231..=291, _, _), _) => BitmapV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => BitmapV1_381_67_09PC,
});
