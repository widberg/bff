use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::MaterialV1_291_03_06PC;

bff_class!(Material {
    (V1_291_03_06, PC) => MaterialV1_291_03_06PC,
});
