pub mod manifest;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_22_pc;

use std::collections::HashMap;
use std::io::{Read, Seek, Write};

use binrw::BinRead;
use serde::Serialize;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::error::UnimplementedVersionPlatformError;
use crate::names::Name;
use crate::platforms::Platform;
use crate::strings::FixedStringNull;
use crate::versions::Version;
use crate::versions::Version::{Asobo, AsoboLegacy};
use crate::BffResult;

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Resource>,
}

impl BigFile {
    pub fn read_platform<R: Read + Seek>(reader: &mut R, platform: Platform) -> BffResult<Self> {
        let version: Version = FixedStringNull::<256>::read_be(reader)?.as_str().into();
        match (version.clone(), platform) {
            (Asobo(1, 6..=699, _, _), _) => {
                v1_06_63_02_pc::read_version_platform(reader, version, platform)
            }
            (AsoboLegacy(_, _), _) => v1_22_pc::read_version_platform(reader, version, platform),
            (version, platform) => {
                Err(UnimplementedVersionPlatformError::new(version, platform).into())
            }
        }
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BffResult<()> {
        match (self.manifest.version.clone(), self.manifest.platform) {
            (Asobo(1, _, _, _), _) => v1_06_63_02_pc::write(self, writer),
            (AsoboLegacy(1, _), _) => v1_22_pc::write(self, writer),
            (version, platform) => {
                Err(UnimplementedVersionPlatformError::new(version, platform).into())
            }
        }
    }
}
