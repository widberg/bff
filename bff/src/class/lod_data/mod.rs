use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;

use v1_06_63_02_pc::LodDataV1_06_63_02PC;

bff_class!(LodData {
    (V1_06_63_02, PC) => LodDataV1_06_63_02PC,
});
