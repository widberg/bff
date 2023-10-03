use bff_derive::bff_class;

mod v1_06_63_02_pc;
use v1_06_63_02_pc::WarpV1_06_63_02PC;

bff_class!(Warp {
    (Asobo(1, 6, 63, 2), PC) | (Asobo(1, 381, 67, 9), PC) => WarpV1_06_63_02PC,
});
