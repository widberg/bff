use crate::names::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum ResourceData {
    Data(Vec<u8>),
    SplitData { link_header: Vec<u8>, body: Vec<u8> },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub class_name: Name,
    pub name: Name,
    pub compress: bool,
    pub data: ResourceData,
}

impl Resource {
    pub fn size(&self) -> usize {
        match &self.data {
            ResourceData::Data(data) => data.len(),
            ResourceData::SplitData { link_header, body } => link_header.len() + body.len(),
        }
    }
}
