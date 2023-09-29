use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::RtcV1_381_67_09PC;

bff_class!(Rtc {
    (V1_381_67_09, PC) => RtcV1_381_67_09PC,
});
