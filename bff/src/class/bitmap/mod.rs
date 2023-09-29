use bff_derive::bff_class;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

use v1_06_63_02_pc::BitmapV1_06_63_02PC;
use v1_291_03_06_pc::BitmapV1_291_03_06PC;
use v1_381_67_09_pc::BitmapV1_381_67_09PC;

bff_class!(Bitmap {
    (V1_06_63_02, PC) => BitmapV1_06_63_02PC,
    (V1_291_03_06, PC) => BitmapV1_291_03_06PC,
    (V1_381_67_09, PC) => BitmapV1_381_67_09PC,
});
