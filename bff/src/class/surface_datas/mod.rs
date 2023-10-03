use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::SurfaceDatasV1_381_67_09PC;

bff_class!(SurfaceDatas {
    (Asobo(1, 381, 67, 9), PC) => SurfaceDatasV1_381_67_09PC,
});
