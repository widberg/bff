use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::LightV1_291_03_06PC;
use v1_381_67_09_pc::LightV1_381_67_09PC;

bff_class!(Light {
    (V1_291_03_06, PC) | (V1_06_63_02, PC) => LightV1_291_03_06PC,
    (V1_381_67_09, PC) => LightV1_381_67_09PC,
});
