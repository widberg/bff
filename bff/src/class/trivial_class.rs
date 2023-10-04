use std::io::Cursor;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::{CompressibleData, Data, ExtendedData};
use crate::error::Error;
use crate::names::Name;
use crate::platforms::Platform;
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;

#[derive(Debug, Serialize, Deserialize)]
pub enum TrivialClassResourceDataType {
    Data,
    ExtendedData { compress: bool },
    CompressibleData { compress: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrivialClass<LinkHeaderType, BodyType> {
    pub class_name: Name,
    pub name: Name,
    pub resource_data_type: TrivialClassResourceDataType,
    pub link_header: LinkHeaderType,
    pub body: BodyType,
}

impl<LinkHeaderType, BodyType> TryFromVersionPlatform<&Resource>
    for TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,

    for<'a> BodyType: BinRead<Args<'a> = (&'a LinkHeaderType,)> + 'a,
{
    type Error = Error;

    fn try_from_version_platform(
        object: &Resource,
        _version: Version,
        platform: Platform,
    ) -> Result<Self, Self::Error> {
        match &object.data {
            ExtendedData {
                compress,
                link_header,
                body,
            } => {
                let mut link_header_cursor = Cursor::new(&link_header);
                let mut body_cursor = Cursor::new(&body);
                let link_header = LinkHeaderType::read_options(
                    &mut link_header_cursor,
                    platform.into(),
                    <LinkHeaderType as binrw::BinRead>::Args::default(),
                )?;
                let body =
                    BodyType::read_options(&mut body_cursor, platform.into(), (&link_header,))?;
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    resource_data_type: TrivialClassResourceDataType::ExtendedData {
                        compress: *compress,
                    },
                    link_header,
                    body,
                })
            }
            Data(data) => {
                let mut data_cursor = Cursor::new(&data);
                let link_header = LinkHeaderType::read_options(
                    &mut data_cursor,
                    platform.into(),
                    <LinkHeaderType as binrw::BinRead>::Args::default(),
                )?;
                let body =
                    BodyType::read_options(&mut data_cursor, platform.into(), (&link_header,))?;
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    resource_data_type: TrivialClassResourceDataType::Data,
                    link_header,
                    body,
                })
            }
            CompressibleData { compress, data } => {
                let mut data_cursor = Cursor::new(&data);
                let link_header = LinkHeaderType::read_options(
                    &mut data_cursor,
                    platform.into(),
                    <LinkHeaderType as binrw::BinRead>::Args::default(),
                )?;
                let body =
                    BodyType::read_options(&mut data_cursor, platform.into(), (&link_header,))?;
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    resource_data_type: TrivialClassResourceDataType::CompressibleData {
                        compress: *compress,
                    },
                    link_header,
                    body,
                })
            }
        }
    }
}

impl<LinkHeaderType, BodyType> TryFromVersionPlatform<&TrivialClass<LinkHeaderType, BodyType>>
    for Resource
where
    for<'a> LinkHeaderType: BinWrite + 'a,
    for<'a> <LinkHeaderType as BinWrite>::Args<'a>: Default,

    for<'a> BodyType: BinWrite + 'a,
    for<'a> <BodyType as BinWrite>::Args<'a>: Default,
{
    type Error = Error;

    fn try_from_version_platform(
        class: &TrivialClass<LinkHeaderType, BodyType>,
        _version: Version,
        platform: Platform,
    ) -> Result<Self, Self::Error> {
        match class.resource_data_type {
            TrivialClassResourceDataType::ExtendedData { compress } => {
                let mut link_header_cursor = Cursor::new(Vec::new());
                let mut body_cursor = Cursor::new(Vec::new());
                LinkHeaderType::write_options(
                    &class.link_header,
                    &mut link_header_cursor,
                    platform.into(),
                    <LinkHeaderType as BinWrite>::Args::default(),
                )?;
                BodyType::write_options(
                    &class.body,
                    &mut body_cursor,
                    platform.into(),
                    <BodyType as BinWrite>::Args::default(),
                )?;
                Ok(Self {
                    class_name: class.class_name,
                    name: class.name,
                    data: ExtendedData {
                        compress,
                        link_header: link_header_cursor.into_inner(),
                        body: body_cursor.into_inner(),
                    },
                })
            }
            TrivialClassResourceDataType::Data => {
                let mut data_cursor = Cursor::new(Vec::new());
                LinkHeaderType::write_options(
                    &class.link_header,
                    &mut data_cursor,
                    platform.into(),
                    <LinkHeaderType as BinWrite>::Args::default(),
                )?;
                BodyType::write_options(
                    &class.body,
                    &mut data_cursor,
                    platform.into(),
                    <BodyType as BinWrite>::Args::default(),
                )?;
                Ok(Self {
                    class_name: class.class_name,
                    name: class.name,
                    data: Data(data_cursor.into_inner()),
                })
            }
            TrivialClassResourceDataType::CompressibleData { compress } => {
                let mut data_cursor = Cursor::new(Vec::new());
                LinkHeaderType::write_options(
                    &class.link_header,
                    &mut data_cursor,
                    platform.into(),
                    <LinkHeaderType as BinWrite>::Args::default(),
                )?;
                BodyType::write_options(
                    &class.body,
                    &mut data_cursor,
                    platform.into(),
                    <BodyType as BinWrite>::Args::default(),
                )?;
                Ok(Self {
                    class_name: class.class_name,
                    name: class.name,
                    data: CompressibleData {
                        compress,
                        data: data_cursor.into_inner(),
                    },
                })
            }
        }
    }
}
