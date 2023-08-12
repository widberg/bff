use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::FontsV1_381_67_09PC;

bff_class!(Fonts {
    (V1_381_67_09, PC) => FontsV1_381_67_09PC,
});
