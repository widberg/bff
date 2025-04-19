use crate::macros::bff_class::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::BinaryV1_381_67_09PC;

bff_class!(Binary {
    (Asobo(1, 381, 67, 9), PC) => BinaryV1_381_67_09PC,
});
