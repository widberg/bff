use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::platforms::Platform;
use super::versions::Version;
use crate::class::Class;
use crate::names::Name;
use crate::BffResult;

#[derive(Debug, Eq, PartialEq)]
pub enum ResourceData {
    Data(Vec<u8>),
    SplitData { link_header: Vec<u8>, body: Vec<u8> },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub class_name: Name,
    pub name: Name,
    pub link_name: Option<Name>,
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

#[derive(Debug, Eq, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little, magic = b"BFF0")] // Increment number when format changes
pub struct BffResourceHeader {
    pub platform: Platform,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BffClass {
    pub header: BffResourceHeader,
    pub class: Class,
}

pub struct BffResource {
    pub header: BffResourceHeader,
    pub resource: Resource,
}

impl BffResource {
    pub fn read<R: Read + Seek>(reader: &mut R) -> BffResult<Self> {
        let header = BffResourceHeader::read(reader)?;
        let resource = Resource::read_resource(reader, header.platform, &header.version)?;
        Ok(Self { header, resource })
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BffResult<()> {
        self.header.write(writer)?;
        self.resource
            .dump_resource(writer, self.header.platform, &self.header.version)?;
        Ok(())
    }
}
