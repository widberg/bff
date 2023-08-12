use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::ParticlesV1_381_67_09PC;

bff_class!(Particles {
    (V1_381_67_09, PC) => ParticlesV1_381_67_09PC,
});
