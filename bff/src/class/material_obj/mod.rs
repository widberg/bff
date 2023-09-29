use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::MaterialObjV1_381_67_09PC;

bff_class!(MaterialObj {
    (V1_381_67_09, PC) => MaterialObjV1_381_67_09PC,
});
