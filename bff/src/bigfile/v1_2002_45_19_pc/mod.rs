pub mod block;
pub mod header;

use std::cmp::max;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use block::*;
use header::*;

use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::*;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::SplitData;
use crate::bigfile::v1_06_63_02_pc::object::Object;
use crate::bigfile::versions::{Version, VersionXple};
use crate::helpers::{calculated_padded, write_align_to};
use crate::lz::lzrs_compress_data_with_header_writer_internal;
use crate::names::NameType::Asobo32;
use crate::names::{Name, NameType};
use crate::traits::BigFileIo;

pub struct BigFileV1_2002_45_19PC;

#[binrw::parser(reader, endian)]
pub fn blocks_parser(
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
                    link_name: None,
                    data: SplitData {
                        link_header: object.link_header.into(),
                        body: object.body.into(),
                    },
                },
            );
        }

        blocks.push(ManifestBlock {
            offset: Some(block_description.working_buffer_offset),
            checksum: None,
            compressed: None,
            objects: block_objects,
        });
    }

    Ok(blocks)
}

impl BigFileIo for BigFileV1_2002_45_19PC {
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
        let len = reader.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(pos, len);

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
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        let begin = writer.stream_position()?;
        writer.seek(SeekFrom::Start(2048))?;

        let mut block_working_buffer_capacity_even = 0u64;
        let mut block_working_buffer_capacity_odd = 0u64;
        let mut block_sector_padding_size = 0u64;

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            let mut calculated_working_buffer_offset = 0usize;

            for object in block.objects.iter() {
                let resource = bigfile.objects.get(&object.name).unwrap();
                let begin_resource = writer.stream_position()?;
                match (&resource.data, object.compress.unwrap_or_default()) {
                    (SplitData { link_header, body }, true) => {
                        let begin_header = writer.stream_position()?;
                        writer.seek(SeekFrom::Current(24))?;
                        writer.write_all(link_header)?;
                        let begin_body = writer.stream_position()?;
                        lzrs_compress_data_with_header_writer_internal(body, writer, endian, ())?;
                        let end_body = writer.stream_position()?;
                        writer.seek(SeekFrom::Start(begin_header))?;
                        let compressed_body_size = (end_body - begin_body) as u32;
                        (link_header.len() as u32 + compressed_body_size).write_options(
                            writer,
                            endian,
                            (),
                        )?;
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        compressed_body_size.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.seek(SeekFrom::Start(end_body))?;

                        let needed_working_buffer_offset =
                            if body.len() > (begin_resource - block_begin) as usize {
                                body.len()
                            } else {
                                0
                            };

                        calculated_working_buffer_offset = max(
                            needed_working_buffer_offset,
                            calculated_working_buffer_offset,
                        );
                    }
                    (SplitData { link_header, body }, false) => {
                        (link_header.len() as u32 + body.len() as u32).write_options(
                            writer,
                            endian,
                            (),
                        )?;
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(link_header)?;
                        writer.write_all(body)?;
                    }
                    _ => todo!(),
                }
            }

            let block_end = writer.stream_position()?;
            let data_size = block_end - block_begin;
            let padding = write_align_to(writer, 2048, 0x00)?;
            let padded_size = data_size + padding as u64;

            block_sector_padding_size += padding as u64;

            let working_buffer_offset = block
                .offset
                .unwrap_or(calculated_padded(calculated_working_buffer_offset, 2048) as u64);

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
                object_count: block.objects.len() as u32,
                padded_size,
                data_size,
                working_buffer_offset,
            });
        }

        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;

        let total_resource_count = block_descriptions
            .iter()
            .map(|x| x.object_count)
            .sum::<u32>();

        let header = Header {
            version_oneple: match bigfile.manifest.version_xple.unwrap_or(0.into()) {
                VersionXple::Oneple(x) | VersionXple::Triple((x, _, _)) => x,
            },
            bigfile_type: bigfile
                .manifest
                .bigfile_type
                .unwrap_or(BigFileType::Normal)
                .into(),
            block_working_buffer_capacity_even,
            block_working_buffer_capacity_odd,
            total_padded_block_size: block_descriptions
                .iter()
                .map(|x| x.padded_size)
                .sum::<u64>(),
            block_descriptions,
            tag: tag.map(|x| x.as_bytes().to_vec()),
            block_sector_padding_size,
            pool_sector_padding_size: 0,
            file_size: end,
            total_decompressed_size: 0,
            zero: 0,
            total_resource_count,
        };
        header.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    const NAME_TYPE: NameType = Asobo32;

    type ResourceType = Object;
}
