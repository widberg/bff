pub mod header;

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult};
use header::{BlockDescription, Header, Resources};

use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::*;
use crate::bigfile::platforms::Platform;
use crate::bigfile::v2_128_92_19_pc::block::*;
use crate::bigfile::v2_128_92_19_pc::resource::Resource;
use crate::bigfile::versions::Version;
use crate::names::NameType::Asobo64;
use crate::names::{Name, NameType};
use crate::traits::BigFileIo;

pub struct BigFileV2_256_38_19PC;

#[binrw::parser(reader, endian)]
pub fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    resources: &mut HashMap<Name, crate::bigfile::resource::Resource>,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        reader.seek(SeekFrom::Start(
            block_description.resources_map_offset as u64 * 2048,
        ))?;
        let block_resource_descriptionss = Resources::read_options(reader, endian, ())?;

        let mut block_resources = Vec::with_capacity(
            block_resource_descriptionss.resources.len()
                + block_resource_descriptionss
                    .data_descriptions
                    .iter()
                    .map(|d| d.resource_count)
                    .sum::<u32>() as usize,
        );
        for resource in block_resource_descriptionss.resources.into_iter() {
            reader.seek(SeekFrom::Start(resource.offset as u64 * 2048))?;
            let resource = Resource::read_options(reader, endian, ())?;

            block_resources.push(ManifestResource {
                name: resource.name,
                compress: Some(resource.compress),
            });

            resources.insert(resource.name, resource.into());
        }

        reader.seek(SeekFrom::Start(
            block_resource_descriptionss.data_offset as u64 * 2048,
        ))?;

        for data_description in block_resource_descriptionss.data_descriptions {
            let data = Data::read_options(reader, endian, (data_description.resource_count,))?;

            for resource in data.resources.into_iter() {
                block_resources.push(ManifestResource {
                    name: resource.name,
                    compress: Some(resource.compress),
                });

                resources.insert(resource.name, resource.into());
            }
        }

        blocks.push(ManifestBlock {
            offset: Some(block_resource_descriptionss.working_buffer_offset as u64),
            checksum: None,
            compress: None,
            resources: block_resources,
        });
    }

    Ok(blocks)
}

impl BigFileIo for BigFileV2_256_38_19PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut resources = HashMap::new();

        let blocks = blocks_parser(
            reader,
            endian,
            (header.block_descriptions.inner, &mut resources),
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
            resources,
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

    type ResourceType = Resource;
}
