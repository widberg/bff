use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;
use v1_291_03_06_pc::BitmapV1_291_03_06PC;

bff_class!(Bitmap {
    (V1_291_03_06, PC) => BitmapV1_291_03_06PC,
});
