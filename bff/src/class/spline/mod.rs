use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
use v1_06_63_02_pc::SplineV1_06_63_02PC;

bff_class!(Spline {
    (V1_06_63_02, PC) => SplineV1_06_63_02PC,
});
