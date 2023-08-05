use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;

use v1_06_63_02_pc::LightDataV1_06_63_02PC;
use v1_291_03_06_pc::LightDataV1_291_03_06PC;

bff_class!(LightData {
    (V1_291_03_06, PC) => LightDataV1_291_03_06PC,
    (V1_06_63_02, PC) => LightDataV1_06_63_02PC,
});
