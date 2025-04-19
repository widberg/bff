use crate::macros::bff_class::bff_class;

mod v1_291_03_06_pc;
mod v1_381_67_09_pc;
use v1_291_03_06_pc::GameObjV1_291_03_06PC;
use v1_381_67_09_pc::GameObjV1_381_67_09PC;

bff_class!(GameObj {
    (Asobo(1, 291, 3, 6), PC) | (Asobo(1, 291, 3, 1), PSP) | (Asobo(1, 6, 63, 2), PC) => GameObjV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => GameObjV1_381_67_09PC,
});
