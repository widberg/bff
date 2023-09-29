use bff_derive::bff_class;

// mod v1_06_63_02_pc;
mod v1_291_03_06_pc;
mod v1_381_67_09_pc;

// use v1_06_63_02_pc::SurfaceV1_06_63_02PC;
use v1_291_03_06_pc::SurfaceV1_291_03_06PC;
use v1_381_67_09_pc::SurfaceV1_381_67_09PC;

bff_class!(Surface {
    // (V1_06_63_02, PC) => SurfaceV1_06_63_02PC,
    (V1_291_03_06, PC) => SurfaceV1_291_03_06PC,
    (V1_381_67_09, PC) => SurfaceV1_381_67_09PC,
});
