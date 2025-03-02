use crate::class::trivial_class::TrivialClass;
use crate::helpers::PascalString;

pub struct UserDefineBodyGeneric {
    pub data: PascalString,
}

pub type UserDefineGeneric = TrivialClass<(), UserDefineBodyGeneric>;
