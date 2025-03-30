use crate::class::trivial_class::TrivialClass;
use crate::helpers::{PascalString, ResourceObjectLinkHeader};

pub struct UserDefineBodyGeneric {
    pub data: PascalString,
}

pub type UserDefineGeneric = TrivialClass<Option<ResourceObjectLinkHeader>, UserDefineBodyGeneric>;
