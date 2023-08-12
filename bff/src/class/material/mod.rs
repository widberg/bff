use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::MaterialV1_06_63_02PC;
use v1_291_03_06_pc::MaterialV1_291_03_06PC;
use v1_381_67_09_pc::MaterialV1_381_67_09PC;

bff_class!(Material {
    (V1_06_63_02, PC) => MaterialV1_06_63_02PC,
    (V1_291_03_06, PC) => MaterialV1_291_03_06PC,
    (V1_381_67_09, PC) => MaterialV1_381_67_09PC,
});
