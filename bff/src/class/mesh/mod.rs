use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;
use v1_291_03_06_pc::MeshV1_291_03_06PC;

bff_class!(Mesh {
    (V1_291_03_06, PC) => MeshV1_291_03_06PC,
});
