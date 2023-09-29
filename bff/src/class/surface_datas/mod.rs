use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::SurfaceDatasV1_381_67_09PC;

bff_class!(SurfaceDatas {
    (V1_381_67_09, PC) => SurfaceDatasV1_381_67_09PC,
});
