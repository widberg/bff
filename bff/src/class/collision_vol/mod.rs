use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::CollisionVolV1_291_03_06PC;

bff_class!(CollisionVol {
    (V1_291_03_06, PC) | (V1_291_03_01, PSP) | (V1_06_63_02, PC) => CollisionVolV1_291_03_06PC
});
