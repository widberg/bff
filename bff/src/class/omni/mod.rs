use crate::macros::bff_class::bff_class;

mod v1_06_63_02_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::OmniV1_06_63_02PC;
use v1_381_67_09_pc::OmniV1_381_67_09PC;

bff_class!(Omni {
    (Asobo(1, 6, 63, 2), PC) => OmniV1_06_63_02PC,
    (Asobo(1, 381, 67, 9), PC) => OmniV1_381_67_09PC,
});
