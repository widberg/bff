use bff_derive::bff_class;

use crate::traits::{TryFromGenericSubstitute, TryIntoSpecific};

mod generic;
mod v1_291_03_06_pc;

use generic::UserDefineGeneric;
use v1_291_03_06_pc::UserDefineV1_291_03_06PC;

bff_class!(#![generic] UserDefine {
    (Asobo(1, _, _, _), _) => UserDefineV1_291_03_06PC,
});
