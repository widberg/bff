pub mod block;
pub mod header;
pub mod resource;

use std::cmp::max;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite};
use block::Block;
use header::*;
use resource::Resource;

use crate::bigfile::BigFile;
use crate::bigfile::manifest::*;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::ResourceData::{Data, SplitData};
use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;
use crate::bigfile::versions::{Version, VersionXple};
use crate::helpers::{calculated_padded, copy_repeat, write_align_to};
use crate::lz::lzrs_compress_data_with_header_writer_internal;
use crate::names::NameType::Asobo32;
use crate::names::{Name, NameType};
use crate::traits::BigFileIo;
use crate::{BffResult, Endian};

#[binrw::parser(reader, endian)]
fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    resources: &mut HashMap<Name, crate::bigfile::resource::Resource>,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        let block = Block::read_options(reader, endian, (&block_description,))?;
        let mut block_resources = Vec::with_capacity(block.resources.len());
        for resource in block.resources.into_iter() {
            block_resources.push(ManifestResource {
                name: resource.name,
                compress: Some(resource.compress),
            });

            resources.insert(resource.name, resource.into());
        }

        blocks.push(ManifestBlock {
            offset: Some(block_description.working_buffer_offset as u64),
            checksum: block_description.checksum,
            compress: None,
            resources: block_resources,
        });
    }

    Ok(blocks)
}

pub struct BigFileV1_08_40_02PC;

impl BigFileIo for BigFileV1_08_40_02PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut resources = HashMap::new();

        let blocks = blocks_parser(reader, endian, (header.block_descriptions, &mut resources))?;

        let pos = reader.stream_position().unwrap();
        let len = reader.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(pos, len);

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_xple: Some(header.version_triple.into()),
                platform,
                bigfile_type: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            resources,
        })
    }

    fn write<W: Write + Seek>(
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        let begin = writer.stream_position()?;
        copy_repeat(writer, 0, 2048 - 256 - 228)?;
        copy_repeat(writer, 0xFF, 228)?;

        let mut block_working_buffer_capacity_even = 0u32;
        let mut block_working_buffer_capacity_odd = 0u32;

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            let mut calculated_working_buffer_offset = 0usize;

            for block_resource in block.resources.iter() {
                let resource = bigfile.resources.get(&block_resource.name).unwrap();
                let begin_resource = writer.stream_position()?;
                match (&resource.data, block_resource.compress.unwrap_or_default()) {
                    (Data(data), true) => {
                        let begin_header = writer.stream_position()?;
                        writer.seek(SeekFrom::Current(16))?;
                        let begin_data = writer.stream_position()?;
                        lzrs_compress_data_with_header_writer_internal(data, writer, endian, ())?;
                        let end_data = writer.stream_position()?;
                        writer.seek(SeekFrom::Start(begin_header))?;
                        (data.len() as u32).write_options(writer, endian, ())?;
                        ((end_data - begin_data) as u32).write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.seek(SeekFrom::Start(end_data))?;

                        let needed_working_buffer_offset =
                            if data.len() > (begin_resource - block_begin) as usize {
                                data.len()
                            } else {
                                0
                            };

                        calculated_working_buffer_offset = max(
                            needed_working_buffer_offset,
                            calculated_working_buffer_offset,
                        );
                    }
                    (Data(data), false) => {
                        (data.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        data.write_options(writer, endian, ())?;
                    }
                    (SplitData { link_header, body }, true) => {
                        let data = [link_header as &[_], body].concat();
                        let begin_header = writer.stream_position()?;
                        writer.seek(SeekFrom::Current(16))?;
                        let begin_data = writer.stream_position()?;
                        lzrs_compress_data_with_header_writer_internal(&data, writer, endian, ())?;
                        let end_data = writer.stream_position()?;
                        writer.seek(SeekFrom::Start(begin_header))?;
                        (data.len() as u32).write_options(writer, endian, ())?;
                        ((end_data - begin_data) as u32).write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.seek(SeekFrom::Start(end_data))?;

                        let needed_working_buffer_offset =
                            if data.len() > (begin_resource - block_begin) as usize {
                                data.len()
                            } else {
                                0
                            };

                        calculated_working_buffer_offset = max(
                            needed_working_buffer_offset,
                            calculated_working_buffer_offset,
                        );
                    }
                    (SplitData { link_header, body }, false) => {
                        ((link_header.len() + body.len()) as u32).write_options(
                            writer,
                            endian,
                            (),
                        )?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        link_header.write_options(writer, endian, ())?;
                        body.write_options(writer, endian, ())?;
                    }
                }
            }

            let block_end = writer.stream_position()?;
            let data_size = (block_end - block_begin) as u32;
            let padding = write_align_to(writer, 2048, 0x00)?;
            let padded_size = data_size + padding as u32;

            let working_buffer_offset = block
                .offset
                .unwrap_or(calculated_padded(calculated_working_buffer_offset, 2048) as u64)
                as u32;

            let block_working_buffer_capacity = padded_size + working_buffer_offset;

            if i % 2 == 0 {
                block_working_buffer_capacity_even = max(
                    block_working_buffer_capacity_even,
                    block_working_buffer_capacity,
                );
            } else {
                block_working_buffer_capacity_odd = max(
                    block_working_buffer_capacity_odd,
                    block_working_buffer_capacity,
                );
            }

            block_descriptions.push(BlockDescription {
                resource_count: block.resources.len() as u32,
                padded_size,
                data_size,
                working_buffer_offset,
                first_resource_name: block.resources.first().map(|r| r.name).unwrap_or_default(),
                // TODO: Calculate checksum using Asobo Alternate on the unpadded block while writing
                checksum: block.checksum,
            });
        }

        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;

        let header = Header {
            block_working_buffer_capacity_even,
            block_working_buffer_capacity_odd,
            total_padded_block_size: end as u32 - 2048,
            version_triple: match bigfile.manifest.version_xple.unwrap_or((0, 0, 0).into()) {
                VersionXple::Oneple(x) => (x, 0, 0),
                VersionXple::Triple(x) => x,
            },
            block_descriptions,
            tag: tag.map(|x| x.as_bytes().to_vec()),
        };
        header.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    const NAME_TYPE: NameType = Asobo32;

    type ResourceType = Resource;
}
