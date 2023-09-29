use bff_derive::bff_class;

mod v1_291_03_06_pc;
mod v1_381_67_09_pc;
use v1_291_03_06_pc::SkinV1_291_03_06PC;
use v1_381_67_09_pc::SkinV1_381_67_09PC;

bff_class!(Skin {
    (V1_291_03_06, PC) | (V1_06_63_02, PC) => SkinV1_291_03_06PC,
    (V1_381_67_09, PC) => SkinV1_381_67_09PC,
});
