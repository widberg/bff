use crate::macros::bff_class::bff_class;

mod generic;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;
use v1_381_67_09_pc::UserDefineV1_381_67_09PC;

bff_class!(#![generic] UserDefine {
    (Asobo(1, 381, 67, 9), PC) => UserDefineV1_381_67_09PC,
    (Asobo(1, _, _, _), _) => UserDefineV1_291_03_06PC,
});
