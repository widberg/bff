use std::io::{BufRead, Cursor};

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::BffError;
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
    pub link_header: LinkHeaderType,
    pub body: BodyType,
}

fn is_eof(cursor: &mut std::io::Cursor<&&Box<[u8]>>) -> bool {
    cursor.fill_buf().map(|buf| buf.is_empty()).unwrap_or(true)
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
                is_eof(&mut link_header_cursor)
                    .then_some(())
                    .ok_or(BffError::UnconsumedInput)?;
                let body =
                    BodyType::read_options(&mut body_cursor, platform.into(), (&link_header,))?;
                is_eof(&mut body_cursor)
                    .then_some(())
                    .ok_or(BffError::UnconsumedInput)?;
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    link_name: object.link_name,
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
                is_eof(&mut data_cursor)
                    .then_some(())
                    .ok_or(BffError::UnconsumedInput)?;
                Ok(Self {
                    class_name: object.class_name,
                    name: object.name,
                    link_name: object.link_name,
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
            data: SplitData {
                link_header: link_header_cursor.into_inner().into(),
                body: body_cursor.into_inner().into(),
            },
        })
    }
}
