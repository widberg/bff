use bff_derive::bff_class;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::LodV1_06_63_02PC;
use v1_291_03_06_pc::LodV1_291_03_06PC;
use v1_381_67_09_pc::LodV1_381_67_09PC;

bff_class!(Lod {
    (V1_06_63_02, PC) => LodV1_06_63_02PC,
    (V1_291_03_06, PC) => LodV1_291_03_06PC,
    (V1_381_67_09, PC) => LodV1_381_67_09PC,
});
