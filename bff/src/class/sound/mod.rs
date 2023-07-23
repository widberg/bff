use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::SoundV1_291_03_06PC;

bff_class!(Sound {
    (V1_291_03_06, PC) => SoundV1_291_03_06PC,
});
