use std::io::Cursor;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::{Data, SplitData};
use crate::bigfile::versions::Version;
use crate::error::Error;
use crate::names::Name;
use crate::traits::TryFromVersionPlatform;

#[derive(Debug, Serialize, Deserialize, ReferencedNames)]
pub struct TrivialClass<LinkHeaderType, BodyType> {
    #[referenced_names(skip)]
    pub class_name: Name,
    #[referenced_names(skip)]
    pub name: Name,
    #[referenced_names(skip)]
    pub link_name: Option<Name>,
    pub compress: bool,
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
            SplitData { link_header, body } => {
                let mut link_header_cursor = Cursor::new(&link_header);
                let mut body_cursor = Cursor::new(&body);
                let link_header = LinkHeaderType::read_options(
                    &mut link_header_cursor,
                    platform.into(),
                    <LinkHeaderType as binrw::BinRead>::Args::default(),
                )?;
                // TODO: Make sure whole link_header was consumed
                let body =
                    BodyType::read_options(&mut body_cursor, platform.into(), (&link_header,))?;
                // TODO: Make sure whole body was consumed
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    link_name: object.link_name,
                    compress: object.compress,
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
                // TODO: Make sure whole data was consumed
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    link_name: object.link_name,
                    compress: object.compress,
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
            link_name: class.link_name,
            compress: class.compress,
            data: SplitData {
                link_header: link_header_cursor.into_inner(),
                body: body_cursor.into_inner(),
            },
        })
    }
}
