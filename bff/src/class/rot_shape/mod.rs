use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;

use v1_06_63_02_pc::RotShapeV1_06_63_02PC;
use v1_291_03_06_pc::RotShapeV1_291_03_06PC;

bff_class!(RotShape {
    (V1_291_03_06, PC) => RotShapeV1_291_03_06PC,
    // (V1_06_63_02, PC) => RotShapeV1_06_63_02PC,
});
