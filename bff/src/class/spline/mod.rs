use bff_derive::bff_class;

mod v1_06_63_02_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::SplineV1_06_63_02PC;
use v1_381_67_09_pc::SplineV1_381_67_09PC;

bff_class!(Spline {
    (V1_06_63_02, PC) => SplineV1_06_63_02PC,
    (V1_381_67_09, PC) => SplineV1_381_67_09PC,
});
