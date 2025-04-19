use crate::macros::bff_class::bff_class;

mod generic;
mod v1_291_03_06_pc;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;

bff_class!(#![generic] UserDefine {
    (Asobo(1, _, _, _), _) => UserDefineV1_291_03_06PC,
});
