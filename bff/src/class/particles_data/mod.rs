use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::ParticlesDataV1_381_67_09PC;

bff_class!(ParticlesData {
    (V1_381_67_09, PC) => ParticlesDataV1_381_67_09PC,
});
