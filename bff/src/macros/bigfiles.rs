macro_rules! bigfiles {
    ($($pattern:pat => $bigfile:ident),* $(,)?) => {
        impl BigFile {
            pub fn probe_name_type_platform<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                _platform: crate::bigfile::platforms::Platform,
                version_override: &Option<crate::bigfile::versions::Version>,
            ) -> crate::BffResult<crate::names::NameType> {
                use binrw::BinRead;

                let start = reader.stream_position()?;
                let version: crate::bigfile::versions::Version =
                    crate::helpers::FixedStringNull::<256>::read_be(reader)?
                        .as_str()
                        .into();
                reader.seek(std::io::SeekFrom::Start(start))?;

                let version = version_override.clone().unwrap_or(version);
                (&version).try_into()
            }

            #[allow(unused_imports)]
            pub fn read_platform<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                platform: crate::bigfile::platforms::Platform,
                version_override: &Option<crate::bigfile::versions::Version>,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<Self> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;
                use binrw::BinRead;

                let _endian: crate::Endian = platform.into();
                let version: crate::bigfile::versions::Version =
                    crate::helpers::FixedStringNull::<256>::read_be(reader)?
                        .as_str()
                        .into();
                let version = version_override.clone().unwrap_or(version);
                match (&version, platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| <$bigfile as BigFileIo>::read(reader, version, platform))
                    })*
                    _ => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn write<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                platform_override: Option<crate::bigfile::platforms::Platform>,
                version_override: &Option<crate::bigfile::versions::Version>,
                version_to_write: &Option<crate::bigfile::versions::Version>,
                tag: Option<&str>,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;
                use binrw::BinWrite;

                let platform = platform_override.unwrap_or(self.manifest.platform);
                let _endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                let version = version_override.as_ref().unwrap_or(version);
                let version_string = if let Some(version_to_write) = version_to_write {
                    version_to_write.to_string()
                } else {
                    version.to_string()
                };
                crate::helpers::FixedStringNull::<256>::write_be(&version_string.into(), writer)?;
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| <$bigfile as BigFileIo>::write(self, writer, tag))
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            #[allow(unused_imports)]
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(
                &self,
                resource: &crate::bigfile::resource::Resource,
                writer: &mut W,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(
                &self,
                resource: &crate::bigfile::resource::Resource,
                writer: &mut W,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                let platform = self.manifest.platform;
                let version = &self.manifest.version;
                crate::bigfile::resource::Resource::dump_bff_resource(
                    resource,
                    writer,
                    platform,
                    version,
                    name_context,
                )
            }

            #[allow(unused_imports)]
            pub fn read_resource<R: std::io::Read + std::io::Seek>(
                &self,
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            #[allow(unused_imports)]
            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(
                &self,
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let crate::bigfile::resource::BffResourceHeader { platform, version } =
                    <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }
        }

        #[allow(unused_imports)]
        impl crate::bigfile::resource::Resource {
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                platform: crate::bigfile::platforms::Platform,
                version: &crate::bigfile::versions::Version,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let endian: crate::Endian = platform.into();
                let resource = self;
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            #[allow(unused_imports)]
            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                platform: crate::bigfile::platforms::Platform,
                version: &crate::bigfile::versions::Version,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let endian: crate::Endian = platform.into();
                <crate::bigfile::resource::BffResourceHeader as binrw::BinWrite>::write(
                    &crate::bigfile::resource::BffResourceHeader {
                        platform,
                        version: version.clone(),
                    },
                    writer,
                )?;
                let resource = self;
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            #[allow(unused_imports)]
            pub fn read_resource<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                platform: crate::bigfile::platforms::Platform,
                version: &crate::bigfile::versions::Version,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }

            #[allow(unused_imports)]
            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let crate::bigfile::resource::BffResourceHeader { platform, version } =
                    <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        if name_context.name_type() != <$bigfile as BigFileIo>::NAME_TYPE {
                            return Err(std::io::Error::other(format!(
                                "NameContext type mismatch: expected {:?}, got {:?}",
                                <$bigfile as BigFileIo>::NAME_TYPE,
                                name_context.name_type()
                            ))
                            .into());
                        }
                        name_context.scope(|| {
                            Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                        })
                    })*
                    (version, platform) => {
                        Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into())
                    }
                }
            }
        }

        impl TryFrom<&crate::bigfile::versions::Version> for crate::names::NameType {
            type Error = crate::BffError;

            fn try_from(
                version: &crate::bigfile::versions::Version,
            ) -> Result<crate::names::NameType, Self::Error> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;
                match (version.clone(), PC) {
                    $($pattern => Ok(<$bigfile as BigFileIo>::NAME_TYPE),)*
                    (version, _platform) => {
                        Err(crate::error::UnimplementedVersionError::new(version).into())
                    }
                }
            }
        }
    }
}

pub(crate) use bigfiles;
