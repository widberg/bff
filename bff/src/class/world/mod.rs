use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::WorldV1_291_03_06PC;

bff_class!(World {
    (V1_291_03_06, PC) => WorldV1_291_03_06PC,
});
