macro_rules! bigfiles {
    ($($version_pattern:pat => $bigfile:ident),* $(,)?) => {
        impl BigFile {
            pub fn read_platform<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                platform: crate::bigfile::platforms::Platform,
                version_override: &Option<crate::bigfile::versions::Version>,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<Self> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;
                use binrw::BinRead;

                let _endian: crate::Endian = platform.into();
                let version: crate::bigfile::versions::Version =
                    crate::helpers::FixedStringNull::<256>::read_be(reader)?
                        .as_str()
                        .into();
                let version = version_override.clone().unwrap_or(version);
                match &version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version).into()),
                }
            }

            pub fn write<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                platform_override: Option<crate::bigfile::platforms::Platform>,
                version_override: &Option<crate::bigfile::versions::Version>,
                version_to_write: &Option<crate::bigfile::versions::Version>,
                tag: Option<&str>,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
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
                match version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version.clone()).into()),
                }
            }

            pub fn dump_resource<W: std::io::Write + std::io::Seek>(
                &self,
                resource: &crate::bigfile::resource::Resource,
                writer: &mut W,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version.clone()).into()),
                }
            }

            pub fn read_resource<R: std::io::Read + std::io::Seek>(
                &self,
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version.clone()).into()),
                }
            }

            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(
                &self,
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let crate::bigfile::resource::BffResourceHeader { platform, version } =
                    <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match &version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version).into()),
                }
            }
        }

        impl crate::bigfile::resource::Resource {
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                platform: crate::bigfile::platforms::Platform,
                version: &crate::bigfile::versions::Version,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let endian: crate::Endian = platform.into();
                let resource = self;
                match version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version.clone()).into()),
                }
            }

            pub fn read_resource<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                platform: crate::bigfile::platforms::Platform,
                version: &crate::bigfile::versions::Version,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let endian: crate::Endian = platform.into();
                match version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version.clone()).into()),
                }
            }

            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;

                let crate::bigfile::resource::BffResourceHeader { platform, version } =
                    <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match &version {
                    $($version_pattern => {
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
                    _ => Err(crate::error::UnimplementedVersionError::new(version).into()),
                }
            }
        }

        impl crate::bigfile::versions::Version {
            pub fn name_type(&self) -> crate::BffResult<crate::names::NameType> {
                use crate::bigfile::versions::Version::*;
                use crate::traits::BigFileIo;
                match self {
                    $($version_pattern => Ok(<$bigfile as BigFileIo>::NAME_TYPE),)*
                    _ => Err(crate::error::UnimplementedVersionError::new(self.clone()).into()),
                }
            }
        }
    }
}

pub(crate) use bigfiles;
