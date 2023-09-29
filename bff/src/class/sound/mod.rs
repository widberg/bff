use bff_derive::bff_class;

mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_291_03_06_pc::SoundV1_291_03_06PC;
use v1_381_67_09_pc::SoundV1_381_67_09PC;

bff_class!(Sound {
    (V1_291_03_06, PC) => SoundV1_291_03_06PC,
    (V1_381_67_09, PC) => SoundV1_381_67_09PC,
});
