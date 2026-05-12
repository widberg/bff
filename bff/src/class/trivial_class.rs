use std::io::{BufRead as _, Cursor};

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::{Data, SplitData};
use crate::bigfile::versions::Version;
use crate::names::{Name, NameContext};
use crate::traits::{FromResource, ToResource};
use crate::{BffError, BffResult};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ReferencedNames)]
pub struct TrivialClass<LinkHeaderType, BodyType> {
    #[referenced_names(skip)]
    pub class_name: Name,
    #[referenced_names(skip)]
    pub name: Name,
    #[referenced_names(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<Name>,
    pub link_header: LinkHeaderType,
    pub body: BodyType,
}

fn is_eof(cursor: &mut std::io::Cursor<&[u8]>) -> bool {
    cursor.fill_buf().map(|buf| buf.is_empty()).unwrap_or(true)
}

impl<LinkHeaderType, BodyType> FromResource for TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinRead + 'a,
    for<'a> <LinkHeaderType as BinRead>::Args<'a>: Default,
    for<'a> BodyType: BinRead<Args<'a> = (&'a LinkHeaderType,)> + 'a,
{
    fn from_resource(
        resource: &Resource,
        _version: &Version,
        platform: Platform,
        name_context: &NameContext,
    ) -> BffResult<Self> {
        name_context.scope(|| match &resource.data {
            SplitData { link_header, body } => {
                let mut link_header_cursor = Cursor::new(link_header as &[u8]);
                let mut body_cursor = Cursor::new(body as &[u8]);
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
                    class_name: resource.class_name,
                    name: resource.name,
                    link_name: resource.link_name,
                    link_header,
                    body,
                })
            }
            Data(data) => {
                let mut data_cursor = Cursor::new(data as &[u8]);
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
                    class_name: resource.class_name,
                    name: resource.name,
                    link_name: resource.link_name,
                    link_header,
                    body,
                })
            }
        })
    }
}

impl<LinkHeaderType, BodyType> ToResource for TrivialClass<LinkHeaderType, BodyType>
where
    for<'a> LinkHeaderType: BinWrite + 'a,
    for<'a> <LinkHeaderType as BinWrite>::Args<'a>: Default,
    for<'a> BodyType: BinWrite + 'a,
    for<'a> <BodyType as BinWrite>::Args<'a>: Default,
{
    fn to_resource(
        &self,
        _version: &Version,
        platform: Platform,
        name_context: &NameContext,
    ) -> BffResult<Resource> {
        name_context.scope(|| {
            let mut link_header_cursor = Cursor::new(Vec::new());
            let mut body_cursor = Cursor::new(Vec::new());
            LinkHeaderType::write_options(
                &self.link_header,
                &mut link_header_cursor,
                platform.into(),
                <LinkHeaderType as BinWrite>::Args::default(),
            )?;
            BodyType::write_options(
                &self.body,
                &mut body_cursor,
                platform.into(),
                <BodyType as BinWrite>::Args::default(),
            )?;
            Ok(Resource {
                class_name: self.class_name,
                name: self.name,
                link_name: self.link_name,
                data: SplitData {
                    link_header: link_header_cursor.into_inner().into(),
                    body: body_cursor.into_inner().into(),
                },
            })
        })
    }
}
