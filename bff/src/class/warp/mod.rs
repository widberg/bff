use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
use v1_06_63_02_pc::WarpV1_06_63_02PC;

bff_class!(Warp {
    (V1_06_63_02, PC) => WarpV1_06_63_02PC,
});