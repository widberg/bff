use bff_derive::bff_class;

mod v1_291_03_06_pc;
mod generic;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;
use generic::UserDefineGeneric;

bff_class!(UserDefine {
    (Asobo(1, 291, 3, 6), PC) | (Asobo(1, 6, 63, 2), PC) | (Asobo(1, 381, 67, 9), PC) => UserDefineV1_291_03_06PC,
});
