pub mod block;
pub mod header;
pub mod object;
pub mod pool;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use block::Block;
use header::*;
use object::Object;
use pool::Pool;

use crate::bigfile::manifest::*;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::SplitData;
use crate::bigfile::v1_06_63_02_pc::pool::{
    calculate_padded_pool_header_size,
    ObjectDescription,
    PoolHeader,
    ReferenceRecord,
};
use crate::bigfile::BigFile;
use crate::helpers::{calculated_padded, write_align_to};
use crate::lz::lzrs_compress_data_with_header_writer_internal;
use crate::names::NameType::Asobo32;
use crate::names::{Name, NameType};
use crate::platforms::Platform;
use crate::traits::BigFileIo;
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
                    compress: object.compress,
                    data: SplitData {
                        link_header: object.link_header,
                        body: object.body,
                    },
                },
            );
        }

        blocks.push(ManifestBlock {
            offset: Some(block_description.working_buffer_offset),
            checksum: block_description.checksum,
            compressed: None,
            objects: block_objects,
        });
    }

    Ok(blocks)
}

#[binrw::parser(reader, endian)]
fn pool_parser(objects: &mut HashMap<Name, Resource>) -> BinResult<ManifestPool> {
    let pool = Pool::read_options(reader, endian, ())?;

    let object_entry_indices = pool.header.object_descriptions_indices.inner;
    let object_entries = pool
        .header
        .object_descriptions
        .iter()
        .map(|x| ManifestPoolObjectEntry {
            name: x.name,
            reference_record_index: x.reference_records_index,
        })
        .collect::<Vec<_>>();
    let reference_records = pool
        .header
        .reference_records
        .iter()
        .map(|x| ManifestPoolReferenceRecord {
            object_entries_starting_index: x.objects_name_starting_index,
            object_entries_count: x.objects_name_count,
        })
        .collect::<Vec<_>>();

    for pool_object in pool.objects.into_iter() {
        let name = pool_object.object.name;
        let object = objects.get_mut(&name).unwrap();
        object.compress = pool_object.object.compress;
        match &mut object.data {
            SplitData { body, .. } => {
                *body = pool_object.object.body;
            }
            _ => unreachable!(),
        }
    }

    Ok(ManifestPool {
        object_entry_indices,
        object_entries,
        reference_records,
    })
}

pub struct BigFileV1_06_63_02PC;

impl BigFileIo for BigFileV1_06_63_02PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let header = Header::read_options(reader, endian, ())?;

        let mut objects = HashMap::new();

        let blocks = blocks_parser(reader, endian, (header.block_descriptions, &mut objects))?;

        let pool = if let Some(pool_offset) = header.pool_offset {
            assert_eq!(pool_offset as u64, reader.stream_position().unwrap());
            Some(pool_parser(reader, endian, (&mut objects,))?)
        } else {
            None
        };

        let pos = reader.stream_position().unwrap();
        let len = reader.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(pos, len);

        Ok(BigFile {
            manifest: Manifest {
                version,
                version_triple: Some(header.version_triple),
                platform,
                rtc: Some(header.is_rtc),
                pool_manifest_unused: header.pool_manifest_unused,
                incredi_builder_string: header.incredi_builder_string,
                blocks,
                pool,
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

        let mut block_working_buffer_capacity_even = 0u32;
        let mut block_working_buffer_capacity_odd = 0u32;
        let mut block_sector_padding_size = 0u32;

        let pooled = if let Some(pool) = &bigfile.manifest.pool {
            let mut pooled = HashSet::new();
            for r in pool.object_entries.iter() {
                pooled.insert(r.name);
            }

            pooled
        } else {
            HashSet::new()
        };

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            let mut calculated_working_buffer_offset = 0usize;

            for object in block.objects.iter() {
                let resource = bigfile.objects.get(&object.name).unwrap();
                let is_pooled = pooled.contains(&object.name);
                let begin_resource = writer.stream_position()?;
                match (&resource.data, is_pooled, resource.compress) {
                    (SplitData { link_header, body }, false, true) => {
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
                    (SplitData { link_header, body }, false, false) => {
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
                    (SplitData { link_header, .. }, true, _) => {
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(link_header)?;
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
                .unwrap_or(calculated_padded(calculated_working_buffer_offset, 2048) as u32);

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
                first_object_name: block.objects.first().map(|r| r.name).unwrap_or_default(),
                // TODO: Calculate checksum using Asobo Alternate on the unpadded block while writing
                checksum: block.checksum,
            });
        }

        let (
            pool_offset,
            pool_sector_padding_size,
            pool_object_decompression_buffer_capacity,
            pool_manifest_padded_size,
        ) = if let Some(pool) = &bigfile.manifest.pool {
            let begin_pool_header = writer.stream_position()?;

            let objects_names_count_sum = pool
                .reference_records
                .iter()
                .map(|x| x.object_entries_count as u32)
                .sum();

            let padded_pool_header_size = calculate_padded_pool_header_size(
                pool.object_entry_indices.len(),
                pool.object_entries.len(),
                pool.reference_records.len(),
            );

            writer.seek(SeekFrom::Current(padded_pool_header_size as i64))?;

            let end_pool_header = writer.stream_position()?;

            let mut object_padded_sizes = HashMap::new();

            let mut pool_sector_padding_size = 0u32;
            let mut pool_object_decompression_buffer_capacity = 0;

            for i in pool.object_entry_indices.iter() {
                let entry = pool.object_entries.get(*i as usize).unwrap();
                let name = entry.name;
                let resource = bigfile.objects.get(&name).unwrap();
                let begin_resource = writer.stream_position()?;
                match (&resource.data, resource.compress) {
                    (SplitData { body, .. }, true) => {
                        let begin_header = writer.stream_position()?;
                        writer.seek(SeekFrom::Current(24))?;
                        let begin_body = writer.stream_position()?;
                        lzrs_compress_data_with_header_writer_internal(body, writer, endian, ())?;
                        let end_body = writer.stream_position()?;
                        writer.seek(SeekFrom::Start(begin_header))?;
                        let compressed_body_size = (end_body - begin_body) as u32;
                        compressed_body_size.write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        compressed_body_size.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.seek(SeekFrom::Start(end_body))?;
                        pool_object_decompression_buffer_capacity = max(
                            (calculated_padded(body.len(), 2048)) / 2048,
                            pool_object_decompression_buffer_capacity,
                        );
                    }
                    (SplitData { body, .. }, false) => {
                        (body.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(body)?;
                        pool_object_decompression_buffer_capacity = max(
                            (calculated_padded(body.len(), 2048)) / 2048,
                            pool_object_decompression_buffer_capacity,
                        );
                    }
                    _ => todo!(),
                }
                let end_resource = writer.stream_position()?;
                let resource_size = (end_resource - begin_resource) as u32;
                let padding = write_align_to(writer, 2048, 0xFF)?;

                pool_sector_padding_size += padding as u32;
                object_padded_sizes.insert(name, (resource_size + padding as u32) / 2048);
            }

            let pool_data_end = writer.stream_position()?;
            writer.seek(SeekFrom::Start(begin_pool_header))?;

            let object_descriptions = pool
                .object_entries
                .iter()
                .map(|x| ObjectDescription {
                    name: x.name,
                    reference_count: pool
                        .object_entry_indices
                        .iter()
                        .filter(|y| pool.object_entries.get(**y as usize).unwrap().name == x.name)
                        .count() as u32,
                    padded_size: *object_padded_sizes.get(&x.name).unwrap(),
                    reference_records_index: x.reference_record_index,
                })
                .collect::<Vec<_>>();

            let start_chunk = (end_pool_header / 2048) as u32;

            let reference_records = pool
                .reference_records
                .iter()
                .map(|x| {
                    let objects_name_starting_index = x.object_entries_starting_index;
                    let objects_name_count = x.object_entries_count;

                    let first = objects_name_starting_index as usize;
                    let last = objects_name_starting_index as usize + objects_name_count as usize;

                    let get_object_padded_size = |x: usize| -> u32 {
                        let object_entry_index = *pool.object_entry_indices.get(x).unwrap();
                        let object_entry = pool
                            .object_entries
                            .get(object_entry_index as usize)
                            .unwrap();
                        *object_padded_sizes.get(&object_entry.name).unwrap()
                    };

                    let start_chunk_index =
                        start_chunk + (0..first).map(get_object_padded_size).sum::<u32>();
                    let end_chunk_index =
                        start_chunk_index + (first..last).map(get_object_padded_size).sum::<u32>();

                    ReferenceRecord {
                        start_chunk_index,
                        end_chunk_index,
                        objects_name_starting_index,
                        objects_name_count,
                    }
                })
                .collect::<Vec<_>>();

            let pool_header = PoolHeader {
                objects_names_count_sum,
                object_descriptions_indices: pool.object_entry_indices.clone().into(),
                object_descriptions,
                reference_records: reference_records.into(),
            };
            pool_header.write_options(writer, endian, ())?;

            let end_pool_header = writer.stream_position()?;
            let pool_manifest_size = (end_pool_header - begin_pool_header) as u32;
            let padding = write_align_to(writer, 2048, 0xFF)?;
            let pool_manifest_padded_size = pool_manifest_size + padding as u32;

            writer.seek(SeekFrom::Start(pool_data_end))?;

            (
                Some(begin_pool_header as u32),
                pool_sector_padding_size,
                pool_object_decompression_buffer_capacity as u32,
                pool_manifest_padded_size / 2048,
            )
        } else {
            <_>::default()
        };

        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;

        let header = Header {
            is_rtc: bigfile.manifest.rtc.unwrap_or(false),
            block_working_buffer_capacity_even,
            block_working_buffer_capacity_odd,
            padded_size: block_descriptions
                .iter()
                .map(|x| x.padded_size)
                .sum::<u32>(),
            version_triple: bigfile.manifest.version_triple.unwrap_or_default(),
            block_descriptions,
            tag: tag.map(|x| x.as_bytes().to_vec()),
            pool_manifest_padded_size,
            pool_offset,
            pool_manifest_unused: bigfile.manifest.pool_manifest_unused,
            pool_object_decompression_buffer_capacity,
            block_sector_padding_size,
            pool_sector_padding_size,
            file_size: end as u32,
            incredi_builder_string: bigfile.manifest.incredi_builder_string.clone(),
        };
        header.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(0x7C0))?;
        writer.write_all(&[0xFF; 0x40])?;

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    const NAME_TYPE: NameType = Asobo32;

    type ResourceType = Object;
}
