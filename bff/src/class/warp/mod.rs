use crate::macros::bff_class::bff_class;

mod v1_06_63_02_pc;
mod v1_381_67_09_pc;
use v1_06_63_02_pc::WarpV1_06_63_02PC;
use v1_381_67_09_pc::WarpV1_381_67_09PC;

bff_class!(Warp {
    (Asobo(1, 381, 67, 9), PC) => WarpV1_381_67_09PC,
    (Asobo(1, _, _, _), _) => WarpV1_06_63_02PC,
});
