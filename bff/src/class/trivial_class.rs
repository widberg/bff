use std::io::Cursor;

use binrw::BinRead;
use serde::Serialize;

use crate::error::Error;
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::TryFromVersionPlatform;
use crate::versions::Version;

#[derive(Debug, Serialize)]
pub struct TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + Serialize + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,

    for<'a> BodyType: BinRead + Serialize + 'a,
    for<'a> <BodyType as BinRead>::Args<'a>: Default,
{
    link_header: LinkHeaderType,
    body: BodyType,
}

impl<LinkHeaderType, BodyType> TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + Serialize + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,

    for<'a> BodyType: BinRead + Serialize + 'a,
    for<'a> <BodyType as BinRead>::Args<'a>: Default,
{
    pub fn link_header(&self) -> &LinkHeaderType {
        &self.link_header
    }

    pub fn body(&self) -> &BodyType {
        &self.body
    }
}

impl<LinkHeaderType, BodyType> TryFromVersionPlatform<&Object>
    for TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + Serialize + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,

    for<'a> BodyType: BinRead + Serialize + 'a,
    for<'a> <BodyType as BinRead>::Args<'a>: Default,
{
    type Error = Error;

    fn try_from_version_platform(
        object: &Object,
        _version: Version,
        platform: Platform,
    ) -> Result<Self, Self::Error> {
        let mut header_cursor = Cursor::new(object.link_header());
        let mut body_cursor = Cursor::new(object.body());
        Ok(Self {
            link_header: LinkHeaderType::read_options(
                &mut header_cursor,
                crate::platforms::platform_to_endian(platform),
                <LinkHeaderType as binrw::BinRead>::Args::default(),
            )?,
            body: BodyType::read_options(
                &mut body_cursor,
                crate::platforms::platform_to_endian(platform),
                <BodyType as binrw::BinRead>::Args::default(),
            )?,
        })
    }
}
