pub mod header;
pub mod pool;

use std::cmp::max;
use std::collections::{HashMap, HashSet, hash_map};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use header::*;
use pool::{Pool, PoolManifest, ResourceDescription, calculate_padded_pool_header_size};

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
use crate::names::NameType::Asobo32;
use crate::names::{Name, NameType};
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

        let mut blocks =
            blocks_parser(reader, endian, (header.block_descriptions, &mut resources))?;

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
        let pooled = bigfile
            .manifest
            .pool
            .as_ref()
            .map_or_else(HashSet::new, |pool| {
                let mut names = HashSet::new();
                for r in pool.resource_entries.iter() {
                    names.insert(r.name);
                }
                names
            });

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());
        let mut compressed = HashMap::new();

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            let mut calculated_working_buffer_offset = 0usize;

            for block_resource in block.resources.iter() {
                let is_compressed = block_resource.compress.unwrap_or_default();
                compressed.insert(block_resource.name, is_compressed);
                let resource = bigfile.resources.get(&block_resource.name).unwrap();
                let begin_resource = writer.stream_position()?;
                let is_pooled = pooled.contains(&block_resource.name);
                match (&resource.data, is_pooled, is_compressed) {
                    (SplitData { link_header, .. }, true, _) => {
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        (link_header.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(link_header)?;
                    }
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

        let _ = writer.stream_position()?;
        let (pool_sector_padding_size, total_decompressed_size) = if let Some(pool) =
            &bigfile.manifest.pool
        {
            let begin_pool_header = writer.stream_position()?;

            let resources_names_count_sum = pool
                .reference_records
                .iter()
                .map(|x| x.resource_entries_count as u32)
                .sum();

            let padded_pool_header_size = calculate_padded_pool_header_size(
                pool.resource_entry_indices.len(),
                pool.resource_entries.len(),
                pool.reference_records.len(),
            );

            writer.seek(SeekFrom::Current(padded_pool_header_size as i64))?;

            let end_pool_header = writer.stream_position()?;

            let mut resource_padded_sizes = HashMap::new();
            let mut pool_sector_padding_size = 0u32;
            let mut pool_resource_decompression_buffer_capacity = 0u32;

            let mut pool_compression_cache = HashMap::new();
            let mut total_decompressed_size = 0u32;

            for i in pool.resource_entry_indices.iter() {
                let entry = pool.resource_entries.get(*i as usize).unwrap();
                let name = entry.name;
                let resource = bigfile.resources.get(&name).unwrap();
                let begin_resource = writer.stream_position()?;
                match (
                    &resource.data,
                    compressed.get(&name).cloned().unwrap_or_default(),
                ) {
                    (SplitData { body, .. }, true) => {
                        let compressed_data = match pool_compression_cache.entry(name) {
                            hash_map::Entry::Occupied(e) => e.into_mut(),
                            hash_map::Entry::Vacant(e) => {
                                let mut compressed_data_writer = Cursor::new(Vec::new());
                                lzrs_compress_data_with_header_writer_internal(
                                    body,
                                    &mut compressed_data_writer,
                                    endian,
                                    (),
                                )?;
                                e.insert(compressed_data_writer.into_inner())
                            }
                        };
                        let compressed_body_size = compressed_data.len() as u32;
                        compressed_body_size.write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        compressed_body_size.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(compressed_data)?;
                        pool_resource_decompression_buffer_capacity = max(
                            u32::try_from((calculated_padded(body.len(), 2048)) / 2048)
                                .unwrap_or(u32::MAX),
                            pool_resource_decompression_buffer_capacity,
                        );
                        total_decompressed_size += body.len() as u32;
                    }
                    (SplitData { body, .. }, false) => {
                        (body.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        (body.len() as u32).write_options(writer, endian, ())?;
                        0u32.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        writer.write_all(body)?;
                        pool_resource_decompression_buffer_capacity = max(
                            u32::try_from((calculated_padded(body.len(), 2048)) / 2048)
                                .unwrap_or(u32::MAX),
                            pool_resource_decompression_buffer_capacity,
                        );
                        total_decompressed_size += body.len() as u32;
                    }
                    _ => todo!(),
                }
                let end_resource = writer.stream_position()?;
                let resource_size = (end_resource - begin_resource) as u32;
                let padding = write_align_to(writer, 2048, 0xFF)?;

                pool_sector_padding_size += padding as u32;
                resource_padded_sizes.insert(name, (resource_size + padding as u32) / 2048);
            }

            let pool_data_end = writer.stream_position()?;
            writer.seek(SeekFrom::Start(begin_pool_header))?;

            let resource_descriptions = pool
                .resource_entries
                .iter()
                .map(|x| ResourceDescription {
                    name: x.name,
                    reference_count: pool
                        .resource_entry_indices
                        .iter()
                        .filter(|y| pool.resource_entries.get(**y as usize).unwrap().name == x.name)
                        .count() as u32,
                    padded_size: *resource_padded_sizes.get(&x.name).unwrap(),
                    reference_records_index: x.reference_record_index,
                })
                .collect::<Vec<_>>();

            let start_chunk = (end_pool_header / 2048) as u32;
            let reference_records = pool
                .reference_records
                .iter()
                .map(|x| {
                    let resource_entries_starting_index = x.resource_entries_starting_index;
                    let resource_entries_count = x.resource_entries_count;

                    let first = resource_entries_starting_index as usize;
                    let last =
                        resource_entries_starting_index as usize + resource_entries_count as usize;

                    let get_resource_padded_size = |x: usize| -> u32 {
                        let resource_entry_index = *pool.resource_entry_indices.get(x).unwrap();
                        let resource_entry = pool
                            .resource_entries
                            .get(resource_entry_index as usize)
                            .unwrap();
                        *resource_padded_sizes.get(&resource_entry.name).unwrap()
                    };

                    let start_chunk_index =
                        start_chunk + (0..first).map(get_resource_padded_size).sum::<u32>();
                    let end_chunk_index = start_chunk_index
                        + (first..last).map(get_resource_padded_size).sum::<u32>();

                    pool::ReferenceRecord {
                        start_chunk_index,
                        end_chunk_index,
                        object_names_starting_index: resource_entries_starting_index,
                        object_names_count: resource_entries_count,
                    }
                })
                .collect::<Vec<_>>();

            let object_names = pool
                .resource_entries
                .iter()
                .map(|x| x.name)
                .collect::<Vec<_>>()
                .into();
            let reference_records_indices = pool
                .resource_entries
                .iter()
                .map(|x| x.reference_record_index)
                .collect::<Vec<_>>()
                .into();
            let object_padded_size = resource_descriptions
                .iter()
                .map(|x| x.padded_size)
                .collect::<Vec<_>>()
                .into();

            let pool_header = PoolManifest {
                object_names_count_sum: resources_names_count_sum,
                object_names_indices: pool.resource_entry_indices.clone().into(),
                object_names,
                resource_descriptions,
                object_padded_size,
                reference_records_indices,
                reference_records: reference_records.into(),
            };
            pool_header.write_options(writer, endian, ())?;

            let _ = writer.stream_position()?;
            writer.seek(SeekFrom::Start(pool_data_end))?;

            (pool_sector_padding_size, total_decompressed_size)
        } else {
            <_>::default()
        };

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
            pool_sector_padding_size,
            file_size: end as u32,
            total_decompressed_size,
            total_resource_count,
        };
        header.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    const NAME_TYPE: NameType = Asobo32;

    type ResourceType = Resource;
}
