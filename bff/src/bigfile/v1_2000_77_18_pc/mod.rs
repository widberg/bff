pub mod header;
pub mod pool;

use std::cmp::max;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use header::*;
use pool::Pool;

use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::*;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::ResourceData::SplitData;
use crate::bigfile::v1_06_63_02_pc::blocks_parser;
use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;
use crate::bigfile::v1_06_63_02_pc::resource::Resource;
use crate::bigfile::versions::{Version, VersionXple};
use crate::helpers::{calculated_padded, write_align_to};
use crate::lz::lzrs_compress_data_with_header_writer_internal;
use crate::names::Name;
use crate::names::NameType;
use crate::names::NameType::Asobo32;
use crate::traits::BigFileIo;

#[binrw::parser(reader, endian)]
fn pool_parser(
    resources: &mut HashMap<Name, crate::bigfile::resource::Resource>,
) -> BinResult<(ManifestPool, HashMap<Name, bool>)> {
    let pool = Pool::read_options(reader, endian, ())?;

    let resource_entry_indices = pool.header.object_names_indices.inner;
    let resource_entries = pool
        .header
        .resource_descriptions
        .iter()
        .map(|x| ManifestPoolResourceEntry {
            name: x.name,
            reference_record_index: x.reference_records_index,
        })
        .collect::<Vec<_>>();
    let reference_records = pool
        .header
        .reference_records
        .iter()
        .map(|x| ManifestPoolReferenceRecord {
            resource_entries_starting_index: x.object_names_starting_index,
            resource_entries_count: x.object_names_count,
        })
        .collect::<Vec<_>>();

    let mut compressed = HashMap::new();

    for pool_resource in pool.resources.into_iter() {
        let name = pool_resource.resource.name;
        let resource = resources.get_mut(&name).unwrap();
        compressed.insert(name, pool_resource.resource.compress);
        match &mut resource.data {
            SplitData { body, .. } => {
                *body = pool_resource.resource.body.into();
            }
            _ => unreachable!(),
        }
    }

    Ok((
        ManifestPool {
            resource_entry_indices,
            resource_entries,
            reference_records,
        },
        compressed,
    ))
}

pub struct BigFileV1_2000_77_18PC;

impl BigFileIo for BigFileV1_2000_77_18PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut resources = HashMap::new();

        let mut blocks = blocks_parser(reader, endian, (header.block_descriptions, &mut resources))?;

        let mut pool = None;
        let end = reader.seek(SeekFrom::End(0)).unwrap();

        if reader.stream_position().unwrap() != end {
            let (parsed_pool, compressed) = pool_parser(reader, endian, (&mut resources,))?;
            for block in blocks.iter_mut() {
                for resource in block.resources.iter_mut() {
                    if let Some(compress) = compressed.get(&resource.name) {
                        resource.compress = Some(*compress);
                    }
                }
            }
            pool = Some(parsed_pool);
        }

        let pos = reader.stream_position().unwrap();
        assert_eq!(pos, end);

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_xple: Some(header.version_oneple.into()),
                platform,
                bigfile_type: Some(header.bigfile_type.into()),
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool,
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
        writer.seek(SeekFrom::Start(2048))?;

        let mut block_working_buffer_capacity_even = 0u32;
        let mut block_working_buffer_capacity_odd = 0u32;
        let mut block_sector_padding_size = 0u32;

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            let mut calculated_working_buffer_offset = 0usize;

            for block_resource in block.resources.iter() {
                let resource = bigfile.resources.get(&block_resource.name).unwrap();
                let begin_resource = writer.stream_position()?;
                match (&resource.data, block_resource.compress.unwrap_or_default()) {
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
            let data_size = (block_end - block_begin) as u32;
            let padding = write_align_to(writer, 2048, 0x00)?;
            let padded_size = data_size + padding as u32;

            block_sector_padding_size += padding as u32;

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

        let total_resource_count = block_descriptions
            .iter()
            .map(|x| x.resource_count)
            .sum::<u32>();

        let header = Header {
            version_oneple: match bigfile.manifest.version_xple.unwrap_or(0.into()) {
                VersionXple::Oneple(x) | VersionXple::Triple((x, _, _)) => x,
            },
            bigfile_type: bigfile
                .manifest
                .bigfile_type
                .unwrap_or(crate::bigfile::manifest::BigFileType::Normal)
                .into(),
            block_working_buffer_capacity_even,
            block_working_buffer_capacity_odd,
            total_padded_block_size: block_descriptions
                .iter()
                .map(|x| x.padded_size)
                .sum::<u32>(),
            block_descriptions,
            tag: tag.map(|x| x.as_bytes().to_vec()),
            block_sector_padding_size,
            pool_sector_padding_size: 0,
            file_size: end as u32,
            total_decompressed_size: 0,
            total_resource_count,
        };
        header.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    const NAME_TYPE: NameType = Asobo32;

    type ResourceType = Resource;
}
