use bff_derive::bff_class;

mod v1_06_63_02_pc;
use v1_06_63_02_pc::WarpV1_06_63_02PC;

bff_class!(Warp {
    (Asobo(1, _, _, _), _) => WarpV1_06_63_02PC,
});
