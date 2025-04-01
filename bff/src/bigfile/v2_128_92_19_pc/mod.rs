pub mod block;
pub mod header;
pub mod object;

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult};
use block::*;
use header::*;
use object::*;

use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::*;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::Resource;
use crate::bigfile::versions::Version;
use crate::names::NameType::Asobo64;
use crate::names::{Name, NameType};
use crate::traits::BigFileIo;

pub struct BigFileV2_128_92_19PC;

#[binrw::parser(reader, endian)]
pub fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    objects: &mut HashMap<Name, Resource>,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        reader.seek(SeekFrom::Start(
            block_description.resources_map_offset as u64 * 2048,
        ))?;
        let resources = Resources::read_options(reader, endian, ())?;

        let mut block_objects = Vec::with_capacity(
            resources.resources.len()
                + resources
                    .data_descriptions
                    .iter()
                    .map(|d| d.resource_count)
                    .sum::<u32>() as usize,
        );
        for object in resources.resources.into_iter() {
            reader.seek(SeekFrom::Start(object.offset as u64 * 2048))?;
            let object = Object::read_options(reader, endian, ())?;

            block_objects.push(ManifestObject {
                name: object.name,
                compress: Some(object.compress),
            });

            objects.insert(object.name, object.into());
        }

        reader.seek(SeekFrom::Start(resources.data_offset as u64 * 2048))?;

        for data_description in resources.data_descriptions {
            let data = Data::read_options(reader, endian, (data_description.resource_count,))?;

            for object in data.objects.into_iter() {
                block_objects.push(ManifestObject {
                    name: object.name,
                    compress: Some(object.compress),
                });

                objects.insert(object.name, object.into());
            }
        }

        blocks.push(ManifestBlock {
            offset: Some(resources.working_buffer_offset as u64),
            checksum: None,
            compressed: None,
            objects: block_objects,
        });
    }

    Ok(blocks)
}

impl BigFileIo for BigFileV2_128_92_19PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut objects = HashMap::new();

        let blocks = blocks_parser(
            reader,
            endian,
            (header.block_descriptions.inner, &mut objects),
        )?;

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_xple: Some(header.version_oneple.into()),
                platform,
                bigfile_type: Some(header.bigfile_type.into()),
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            objects,
        })
    }

    fn write<W: Write + Seek>(
        _bigfile: &BigFile,
        _writer: &mut W,
        _tag: Option<&str>,
    ) -> BffResult<()> {
        todo!()
    }

    const NAME_TYPE: NameType = Asobo64;

    type ResourceType = Object;
}
