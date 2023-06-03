use binrw::BinRead;
use serde::Serialize;

use crate::object::Object;

// Sorry about the magic numbers!
// Rust does not have good compile time execution support
// so we can't run the hash function at compile time.
// And I'm not going to hash the class names every time
// we parse a class.
#[derive(BinRead, Serialize, Debug)]
#[serde(untagged)]
#[br(import(object_ptr: Object))]
pub enum Class {
    #[br(pre_assert(object_ptr.class_name() == 0x52F79F96))]
    UserDefine(),
}
