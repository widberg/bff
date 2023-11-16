pub mod block;
pub mod header;
pub mod object;

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult};
use block::*;
use header::*;
use object::*;

use crate::bigfile::manifest::*;
use crate::bigfile::resource::Resource;
use crate::bigfile::BigFile;
use crate::names::NameType::Asobo64;
use crate::names::{Name, NameType};
use crate::platforms::Platform;
use crate::traits::BigFileIo;
use crate::versions::Version;
use crate::BffResult;

pub struct BigFileV2_128_52_19PC;

#[binrw::parser(reader, endian)]
pub fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    objects: &mut HashMap<Name, Resource>,
    local_objects: bool,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        reader.seek(SeekFrom::Start(
            block_description.resources_map_offset as u64 * 16,
        ))?;
        let resources = Resources::read_options(reader, endian, ())?;

        let mut block_objects = Vec::with_capacity(
            resources.resources.len()
                + resources.resources2.len()
                + resources
                    .data_descriptions
                    .iter()
                    .map(|d| d.resource_count)
                    .sum::<u32>() as usize,
        );
        for object in resources.resources.into_iter() {
            if local_objects {
                reader.seek(SeekFrom::Start(object.offset as u64 * 16))?;
                let object = Object::read_options(reader, endian, ())?;

                block_objects.push(ManifestObject {
                    name: object.name,
                    compress: Some(object.compress),
                });

                objects.insert(object.name, object.into());
            } else {
                // TODO: Look this up in common
                block_objects.push(ManifestObject {
                    name: object.name,
                    compress: None,
                });
            }
        }

        for object in resources.resources2.into_iter() {
            if local_objects {
                reader.seek(SeekFrom::Start(object.offset as u64 * 16))?;
                let object = Object::read_options(reader, endian, ())?;

                block_objects.push(ManifestObject {
                    name: object.name,
                    compress: Some(object.compress),
                });

                objects.insert(object.name, object.into());
            } else {
                // TODO: Look this up in common
                block_objects.push(ManifestObject {
                    name: object.name,
                    compress: None,
                });
            }
        }

        reader.seek(SeekFrom::Start(resources.data_offset as u64 * 16))?;

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

impl BigFileIo for BigFileV2_128_52_19PC {
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
            (header.block_descriptions.inner, &mut objects, header.resources_block_offset != 0 && header.resources_block_size != 0 && header.map_offset == 0 && header.map_size == 0),
        )?;

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_xple: Some((header.version_oneple as u32).into()),
                platform,
                rtc: Some(header.is_rtc),
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