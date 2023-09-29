use bff_derive::bff_class;

mod v1_381_67_09_pc;
use v1_381_67_09_pc::CameraV1_381_67_09PC;

bff_class!(Camera {
    (V1_381_67_09, PC) => CameraV1_381_67_09PC,
});
