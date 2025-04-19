use crate::macros::bff_class::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::ParticlesDataV1_381_67_09PC;

bff_class!(ParticlesData {
    (Asobo(1, 381, 67, 9), PC) => ParticlesDataV1_381_67_09PC,
});
