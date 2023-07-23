use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::LightV1_291_03_06PC;

bff_class!(Light {
    (V1_291_03_06, PC) => LightV1_291_03_06PC,
});
