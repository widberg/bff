use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;

use v1_06_63_02_pc::LodDataV1_06_63_02PC;
use v1_291_03_06_pc::LodDataV1_291_03_06PC;

bff_class!(LodData {
    (V1_291_03_06, PC) => LodDataV1_291_03_06PC,
    (V1_06_63_02, PC) => LodDataV1_06_63_02PC,
});
