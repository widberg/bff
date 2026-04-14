use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinWrite, binrw};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::platforms::Platform;
use super::versions::Version;
use crate::BffResult;
use crate::class::Class;
use crate::names::{Name, NameContext, NameType};

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
    #[brw(align_after = 16)]
    pub version: Version,
}

impl BffResourceHeader {
    pub fn name_type(&self) -> BffResult<NameType> {
        (&self.version).try_into()
    }

    pub fn probe_name_type<R: Read + Seek>(reader: &mut R) -> BffResult<NameType> {
        let start = reader.stream_position()?;
        let header = Self::read(reader)?;
        reader.seek(SeekFrom::Start(start))?;
        header.name_type()
    }

    fn data_padded_size_on_disk(&self) -> u16 {
        let non_data_size = 4 + 2;
        let total_unpadded_size =
            non_data_size + self.platform.size_on_disk() + self.version.size_on_disk();
        let padding_size = (16 - (total_unpadded_size % 16)) % 16;
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
    pub fn read<R: Read + Seek>(reader: &mut R, name_context: &NameContext) -> BffResult<Self> {
        let header = BffResourceHeader::read(reader)?;
        let resource =
            Resource::read_resource(reader, header.platform, &header.version, name_context)?;
        Ok(Self { header, resource })
    }

    pub fn write<W: Write + Seek>(
        &self,
        writer: &mut W,
        name_context: &NameContext,
    ) -> BffResult<()> {
        self.header.write(writer)?;
        self.resource.dump_resource(
            writer,
            self.header.platform,
            &self.header.version,
            name_context,
        )?;
        Ok(())
    }
}
