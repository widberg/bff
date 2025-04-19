use crate::macros::bff_class::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::FontsV1_381_67_09PC;

bff_class!(Fonts {
    (Asobo(1, 381, 67, 9), PC) => FontsV1_381_67_09PC,
});
