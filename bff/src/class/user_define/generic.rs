use crate::class::trivial_class::TrivialClass;
use crate::helpers::{PascalString, ResourceLinkHeader};

pub struct UserDefineBodyGeneric {
    pub data: PascalString,
}

pub type UserDefineGeneric = TrivialClass<Option<ResourceLinkHeader>, UserDefineBodyGeneric>;
