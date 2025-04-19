macro_rules! bigfiles {
    ($($pattern:pat => $bigfile:ident),* $(,)?) => {
        impl BigFile {
            #[allow(unused_imports)]
            pub fn read_platform<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::bigfile::platforms::Platform, version_override: &Option<crate::bigfile::versions::Version>) -> crate::BffResult<Self> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use binrw::BinRead;
                use crate::traits::BigFileIo;
                let _endian: crate::Endian = platform.into();
                let version: crate::bigfile::versions::Version = crate::helpers::FixedStringNull::<256>::read_be(reader)?.as_str().into();
                let version = version_override.clone().unwrap_or(version);
                match (&version, platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        <$bigfile as BigFileIo>::read(reader, version, platform)
                    })*
                    _ => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn write<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform_override: Option<crate::bigfile::platforms::Platform>, version_override: &Option<crate::bigfile::versions::Version>, tag: Option<&str>) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use binrw::BinWrite;
                use crate::traits::BigFileIo;
                let platform = platform_override.unwrap_or(self.manifest.platform);
                let _endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                let version = version_override.as_ref().unwrap_or(version);
                let version_string = version.to_string();
                crate::helpers::FixedStringNull::<256>::write_be(&version_string.into(), writer)?;
                match (version.clone(), platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        <$bigfile as BigFileIo>::write(self, writer, tag)
                    })*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(&self, resource: &crate::bigfile::resource::Resource, writer: &mut W) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match (version.clone(), platform) {
                    $($pattern => {crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)})*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(&self, resource: &crate::bigfile::resource::Resource, writer: &mut W) -> crate::BffResult<()> {
                let platform = self.manifest.platform;
                let version = &self.manifest.version;
                crate::bigfile::resource::Resource::dump_bff_resource(resource, writer, platform, version)
            }

            #[allow(unused_imports)]
            pub fn read_resource<R: std::io::Read + std::io::Seek>(&self, reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let platform = self.manifest.platform;
                let endian: crate::Endian = platform.into();
                let version = &self.manifest.version;
                match (version.clone(), platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                    })*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(&self, reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version,
                } = <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                    })*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }
        }

        #[allow(unused_imports)]
        impl crate::bigfile::resource::Resource {
            pub fn dump_resource<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                let resource = self;
                match (version.clone(), platform) {
                    $($pattern => {crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)})*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn dump_bff_resource<W: std::io::Write + std::io::Seek>(&self, writer: &mut W, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<()> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                <crate::bigfile::resource::BffResourceHeader as binrw::BinWrite>::write(&crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version: version.clone(),
                }, writer)?;
                let resource = self;
                match (version.clone(), platform) {
                    $($pattern => {crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::dump_resource(resource, writer, endian)?)})*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn read_resource<R: std::io::Read + std::io::Seek>(reader: &mut R, platform: crate::bigfile::platforms::Platform, version: &crate::bigfile::versions::Version) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                    })*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }

            #[allow(unused_imports)]
            pub fn read_bff_resource<R: std::io::Read + std::io::Seek>(reader: &mut R) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                let crate::bigfile::resource::BffResourceHeader {
                    platform,
                    version,
                } = <crate::bigfile::resource::BffResourceHeader as binrw::BinRead>::read(reader)?;
                let endian: crate::Endian = platform.into();
                match (version.clone(), platform) {
                    $($pattern => {
                        crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        Ok(<$bigfile as BigFileIo>::ResourceType::read_resource(reader, endian)?)
                    })*
                    (version, platform) => Err(crate::error::UnimplementedVersionPlatformError::new(version, platform).into()),
                }
            }
        }

        #[allow(non_snake_case)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct BigFileTryYourBestReport {
            pub total: usize,
            $($bigfile: usize),*
        }

        impl<R: std::io::Read + std::io::Seek> crate::traits::TryYourBest<&mut R> for BigFile {
            type Report = BigFileTryYourBestReport;
            fn update_report(reader: &mut R, platform: crate::bigfile::platforms::Platform, report: &mut Self::Report) {
                use crate::traits::BigFileIo;
                report.total += 1;
                // TODO: Probably need a way to do this without specifying a version.
                $(
                    reader.seek(std::io::SeekFrom::Start(256)).unwrap();
                    report.$bigfile += {crate::names::names().lock().unwrap().name_type = <$bigfile as BigFileIo>::NAME_TYPE;
                        <bool as Into<usize>>::into(<$bigfile as BigFileIo>::read(reader, crate::bigfile::versions::Version::Asobo(0, 0, 0, 0), platform).is_ok())};
                )*
                reader.rewind().unwrap();
            }
        }

        impl std::fmt::Display for BigFileTryYourBestReport {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "BigFile")?;
                writeln!(f, "Total: {}", self.total)?;
                $(
                    writeln!(f, "{}: {}", stringify!($bigfile), self.$bigfile)?;
                )*
                Ok(())
            }
        }

        impl TryFrom<&crate::bigfile::versions::Version> for crate::names::NameType {
            type Error = crate::BffError;

            fn try_from(version: &crate::bigfile::versions::Version) -> Result<crate::names::NameType, Self::Error> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                use crate::traits::BigFileIo;
                match (version.clone(), PC) {
                    $($pattern => Ok(<$bigfile as BigFileIo>::NAME_TYPE),)*
                    (version, _platform) => Err(crate::error::UnimplementedVersionError::new(version).into()),
                }
            }
        }
    }
}

pub(crate) use bigfiles;
