use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::AnimationV1_291_03_06PC;

bff_class!(Animation {
    (V1_291_03_06, PC) => AnimationV1_291_03_06PC,
});
