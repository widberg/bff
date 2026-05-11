use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinWrite, binrw};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::platforms::Platform;
use super::versions::Version;
use crate::BffResult;
use crate::class::Class;
use crate::names::{Name, NameContext, NameType};
use crate::traits::{FromResource, ToResource};

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
        self.version.name_type()
    }

    pub fn probe_name_type<R: Read + Seek>(reader: &mut R) -> BffResult<NameType> {
        let start = reader.stream_position()?;
        let header = Self::read(reader)?;
        reader.seek(SeekFrom::Start(start))?;
        header.name_type()
    }

    const fn data_padded_size_on_disk(&self) -> u16 {
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

pub struct BffResourceRef<'a> {
    pub platform: Platform,
    pub version: &'a Version,
    pub resource: &'a Resource,
}

impl BffClass {
    pub fn bff_resource(&self, name_context: &NameContext) -> BffResult<BffResource> {
        self.bff_resource_with_override(None, None, name_context)
    }

    pub fn bff_resource_with_override(
        &self,
        platform_override: Option<Platform>,
        version_override: Option<&Version>,
        name_context: &NameContext,
    ) -> BffResult<BffResource> {
        let platform = platform_override.unwrap_or(self.header.platform);
        let version = version_override.unwrap_or(&self.header.version);
        let resource = self.class.to_resource(version, platform, name_context)?;
        Ok(BffResource {
            header: BffResourceHeader {
                platform,
                version: version.clone(),
            },
            resource,
        })
    }
}

impl BffResource {
    pub const fn as_ref(&self) -> BffResourceRef<'_> {
        BffResourceRef {
            platform: self.header.platform,
            version: &self.header.version,
            resource: &self.resource,
        }
    }

    pub fn bff_class(&self, name_context: &NameContext) -> BffResult<BffClass> {
        self.bff_class_with_override(None, None, name_context)
    }

    pub fn bff_class_with_override(
        &self,
        platform_override: Option<Platform>,
        version_override: Option<&Version>,
        name_context: &NameContext,
    ) -> BffResult<BffClass> {
        let platform = platform_override.unwrap_or(self.header.platform);
        let version = version_override.unwrap_or(&self.header.version);
        BffResourceRef {
            platform,
            version,
            resource: &self.resource,
        }
        .bff_class(name_context)
    }

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

impl BffResourceRef<'_> {
    pub fn header(&self) -> BffResourceHeader {
        BffResourceHeader {
            platform: self.platform,
            version: self.version.clone(),
        }
    }

    pub fn bff_class(&self, name_context: &NameContext) -> BffResult<BffClass> {
        let class = Class::from_resource(self.resource, self.version, self.platform, name_context)?;
        Ok(BffClass {
            header: self.header(),
            class,
        })
    }

    pub fn write<W: Write + Seek>(
        &self,
        writer: &mut W,
        name_context: &NameContext,
    ) -> BffResult<()> {
        self.header().write(writer)?;
        self.resource
            .dump_resource(writer, self.platform, self.version, name_context)?;
        Ok(())
    }
}
