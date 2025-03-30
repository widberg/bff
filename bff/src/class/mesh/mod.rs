use bff_derive::bff_class;

pub mod generic;
pub mod v1_06_63_02_pc;
pub mod v1_291_03_06_pc;
pub mod v1_381_67_09_pc;

use v1_06_63_02_pc::MeshV1_06_63_02PC;
use v1_291_03_06_pc::MeshV1_291_03_06PC;
use v1_381_67_09_pc::MeshV1_381_67_09PC;

bff_class!(Mesh {
    (Asobo(1, 6, 63, 2), PC) => MeshV1_06_63_02PC,
    (Asobo(1, 291, 3, 6), PC) => MeshV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => MeshV1_381_67_09PC,
});
