use bff_derive::bff_class;

mod generic;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::LightV1_291_03_06PC;
use v1_381_67_09_pc::LightV1_381_67_09PC;

bff_class!(Light {
    (Asobo(1, 3..=291, _, _), _) => LightV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => LightV1_381_67_09PC,
});
