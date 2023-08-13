use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;
mod v1_381_67_09_pc;
use v1_291_03_06_pc::GameObjV1_291_03_06PC;
use v1_381_67_09_pc::GameObjV1_381_67_09PC;

bff_class!(GameObj {
    (V1_291_03_06, PC) | (V1_291_03_01, PSP) | (V1_06_63_02, PC) => GameObjV1_291_03_06PC,
    (V1_381_67_09, PC) => GameObjV1_381_67_09PC,
});
