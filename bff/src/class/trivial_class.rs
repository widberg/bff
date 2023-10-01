use std::io::Cursor;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::names::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrivialClass<LinkHeaderType, BodyType> {
    pub name: Name,
    pub class_name: Name,
    pub link_header: LinkHeaderType,
    pub body: BodyType,
}

impl<LinkHeaderType, BodyType> TryFromVersionPlatform<&Object>
    for TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,

    for<'a> BodyType: BinRead<Args<'a> = (&'a LinkHeaderType,)> + 'a,
{
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> Result<Self, Self::Error> {
        let mut link_header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        let link_header = LinkHeaderType::read_options(
            &mut link_header_cursor,
            platform.into(),
            <LinkHeaderType as binrw::BinRead>::Args::default(),
        )?;
        let body = BodyType::read_options(&mut body_cursor, platform.into(), (&link_header,))?;
        Ok(Self {
            name: object.name,
            class_name: object.class_name,
            link_header,
            body,
        })
    }
}

impl<LinkHeaderType, BodyType> TryFromVersionPlatform<&TrivialClass<LinkHeaderType, BodyType>>
    for Object
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
            compress: false,
            name: class.name,
            class_name: class.class_name,
            link_header: link_header_cursor.into_inner(),
            body: body_cursor.into_inner(),
        })
    }
}
