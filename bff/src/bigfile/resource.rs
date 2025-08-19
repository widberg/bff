use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinWrite, binrw};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::platforms::Platform;
use super::versions::Version;
use crate::BffResult;
use crate::class::Class;
use crate::names::Name;

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

impl Resource {
    pub fn size(&self) -> usize {
        match &self.data {
            ResourceData::Data(data) => data.len(),
            ResourceData::SplitData { link_header, body } => link_header.len() + body.len(),
        }
    }
}

#[binrw]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[brw(little, magic = b"BFF0")] // Increment number when format changes
pub struct BffResourceHeader {
    #[br(temp)]
    #[bw(calc = self.data_padded_size_on_disk())]
    _size: u16,
    pub platform: Platform,
    #[brw(align_after = 0x10)]
    pub version: Version,
}

impl BffResourceHeader {
    fn data_padded_size_on_disk(&self) -> u16 {
        let non_data_size = 4 + 2;
        let total_unpadded_size =
            non_data_size + self.platform.size_on_disk() + self.version.size_on_disk();
        let padding_size = total_unpadded_size % 0x10;
        let total_padded_size = total_unpadded_size + padding_size;
        total_padded_size - non_data_size
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
