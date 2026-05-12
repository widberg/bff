use crate::names::Name;

pub mod bff_resource;

#[derive(Debug, Eq, PartialEq)]
pub enum ResourceData {
    Data(Box<[u8]>),
    SplitData {
        link_header: Box<[u8]>,
        body: Box<[u8]>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub class_name: Name,
    pub name: Name,
    pub link_name: Option<Name>,
    pub data: ResourceData,
}
