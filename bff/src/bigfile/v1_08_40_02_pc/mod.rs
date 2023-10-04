pub mod block;
pub mod header;
pub mod object;

use std::collections::HashMap;
use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinResult};
use block::Block;
use header::*;

use crate::bigfile::manifest::*;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::CompressibleData;
use crate::bigfile::BigFile;
use crate::names::Name;
use crate::platforms::Platform;
use crate::traits::{BigFileRead, BigFileWrite};
use crate::versions::Version;
use crate::BffResult;

#[binrw::parser(reader, endian)]
fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    objects: &mut HashMap<Name, Resource>,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        let block = Block::read_options(reader, endian, (&block_description,))?;
        let mut block_objects = Vec::with_capacity(block.objects.len());
        for object in block.objects.into_iter() {
            block_objects.push(ManifestObject {
                name: object.name,
                compress: Some(object.compress),
            });

            objects.insert(
                object.name,
                Resource {
                    class_name: object.class_name,
                    name: object.name,
                    data: CompressibleData {
                        compress: object.compress,
                        data: object.data,
                    },
                },
            );
        }

        blocks.push(ManifestBlock {
            offset: Some(block_description.working_buffer_offset),
            checksum: block_description.checksum,
            objects: block_objects,
        });
    }

    Ok(blocks)
}

pub struct BigFileV1_08_40_02PC;

impl BigFileRead for BigFileV1_08_40_02PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut objects = HashMap::new();

        let blocks = blocks_parser(reader, endian, (header.block_descriptions, &mut objects))?;

        let pos = reader.stream_position().unwrap();
        let len = reader.seek(std::io::SeekFrom::End(0)).unwrap();
        assert_eq!(pos, len);

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_triple: Some(header.version_triple),
                platform,
                rtc: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            objects,
        })
    }
}

impl BigFileWrite for BigFileV1_08_40_02PC {
    fn write<W: Write + Seek>(_bigfile: &BigFile, _writer: &mut W) -> BffResult<()> {
        todo!()
    }
}
