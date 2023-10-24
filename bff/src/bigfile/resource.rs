use crate::names::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum ResourceData {
    Data(Vec<u8>),
    CompressibleData {
        compress: bool,
        data: Vec<u8>,
    },
    ExtendedData {
        compress: bool,
        link_header: Vec<u8>,
        body: Vec<u8>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub class_name: Name,
    pub name: Name,
    pub data: ResourceData,
}

impl Resource {
    pub fn size(&self) -> usize {
        match &self.data {
            ResourceData::Data(data) | ResourceData::CompressibleData { data, .. } => data.len(),
            ResourceData::ExtendedData {
                link_header, body, ..
            } => link_header.len() + body.len(),
        }
    }
}
