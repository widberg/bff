use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

pub mod v1_06_63_02_pc;
pub mod v1_291_03_06_pc;
pub mod v1_381_67_09_pc;

use v1_06_63_02_pc::MeshV1_06_63_02PC;
use v1_291_03_06_pc::MeshV1_291_03_06PC;
use v1_381_67_09_pc::MeshV1_381_67_09PC;

bff_class!(Mesh {
    (V1_06_63_02, PC) => MeshV1_06_63_02PC,
    (V1_291_03_06, PC) => MeshV1_291_03_06PC,
    (V1_381_67_09, PC) => MeshV1_381_67_09PC,
});
