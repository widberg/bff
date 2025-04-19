use crate::macros::bff_class::bff_class;

pub mod v1_291_03_06_pc;
pub mod v1_381_67_09_pc;
use v1_291_03_06_pc::SkinV1_291_03_06PC;
use v1_381_67_09_pc::SkinV1_381_67_09PC;

bff_class!(Skin {
    (Asobo(1, 291, 3, 6), PC) | (Asobo(1, 6, 63, 2), PC) => SkinV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => SkinV1_381_67_09PC,
});
