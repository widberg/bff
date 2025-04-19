use crate::macros::bff_class::bff_class;

mod v1_06_63_02_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::SplineV1_06_63_02PC;
use v1_381_67_09_pc::SplineV1_381_67_09PC;

bff_class!(Spline {
    (Asobo(1, 6, 63, 2), PC) => SplineV1_06_63_02PC,
    (Asobo(1, 381, 67, 9), PC) => SplineV1_381_67_09PC,
});
