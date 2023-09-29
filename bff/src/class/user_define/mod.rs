use bff_derive::bff_class;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;

bff_class!(UserDefine {
    (V1_291_03_06, PC) | (V1_06_63_02, PC) | (V1_381_67_09, PC) => UserDefineV1_291_03_06PC,
});
