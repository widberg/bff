use bff_derive::bff_class;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::LodDataV1_06_63_02PC;
use v1_291_03_06_pc::LodDataV1_291_03_06PC;
use v1_381_67_09_pc::LodDataV1_381_67_09PC;

bff_class!(LodData {
    (Asobo(1, 6, 63, 2), PC) => LodDataV1_06_63_02PC,
    (Asobo(1, 291, 3, 6), PC) => LodDataV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => LodDataV1_381_67_09PC,
});
