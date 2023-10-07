pub mod block;
pub mod header;
pub mod object;
pub mod pool;

use std::cmp::max;
use std::collections::HashMap;
use std::default::Default;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use block::Block;
use header::*;
use pool::Pool;

use crate::bigfile::manifest::*;
use crate::bigfile::resource::ResourceData::ExtendedData;
use crate::bigfile::resource::{Resource, ResourceData};
use crate::bigfile::BigFile;
use crate::lz::compress_data_with_header_writer_internal;
use crate::names::Name;
use crate::platforms::Platform;
use crate::traits::{BigFileRead, BigFileWrite};
use crate::versions::{Version, VersionTriple};
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
                    data: ExtendedData {
                        compress: object.compress,
                        link_header: object.link_header,
                        body: object.body,
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
        match &mut objects.get_mut(&name).unwrap().data {
            ExtendedData { ref mut body, .. } => *body = pool_object.object.body,
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

impl BigFileRead for BigFileV1_06_63_02PC {
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
}

impl BigFileWrite for BigFileV1_06_63_02PC {
    fn write<W: Write + Seek>(bigfile: &BigFile, writer: &mut W) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        let begin = writer.stream_position()?;
        writer.seek(SeekFrom::Start(2048))?;

        let mut block_working_buffer_capacity_even = 0u32;
        let mut block_working_buffer_capacity_odd = 0u32;
        let mut block_sector_padding_size = 0u32;

        let mut block_descriptions = Vec::with_capacity(bigfile.manifest.blocks.len());

        for (i, block) in bigfile.manifest.blocks.iter().enumerate() {
            let block_begin = writer.stream_position()?;

            for object in block.objects.iter() {
                let resource = bigfile.objects.get(&object.name).unwrap();
                match resource.data {
                    ResourceData::ExtendedData {
                        compress,
                        ref link_header,
                        ref body,
                    } => {
                        if compress {
                            let begin_header = writer.stream_position()?;
                            writer.seek(SeekFrom::Current(24))?;
                            let begin_body = writer.stream_position()?;
                            writer.write_all(link_header)?;
                            compress_data_with_header_writer_internal(body, writer, endian, ())?;
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
                        } else {
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
                    }
                    _ => todo!(),
                }
            }

            let block_end = writer.stream_position()?;
            let data_size = (block_end - block_begin) as u32;
            let padding = vec![0x00; (2048 - (data_size % 2048)) as usize];
            writer.write_all(&padding)?;
            let padded_size = data_size + padding.len() as u32;

            block_sector_padding_size += padding.len() as u32;

            let working_buffer_offset = block.offset.unwrap_or(0);

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
                first_object_name: block
                    .objects
                    .first()
                    .map(|r| r.name)
                    .unwrap_or(Name::default()),
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

            let pool_offset = Some(begin_pool_header as u32);
            let pool_sector_padding_size = 0u32;
            let pool_object_decompression_buffer_capacity = 0u32;

            // TODO: Write pool header

            let end_pool_header = writer.stream_position()?;
            let pool_manifest_padded_size = (end_pool_header - begin_pool_header) as u32;

            for i in pool.object_entry_indices.iter() {
                let x = pool.object_entries.get(*i as usize).unwrap();
                let _ = pool
                    .reference_records
                    .get(x.reference_record_index as usize)
                    .unwrap();
            }

            (
                pool_offset,
                pool_sector_padding_size,
                pool_object_decompression_buffer_capacity,
                pool_manifest_padded_size,
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
            padded_size: end as u32,
            version_triple: bigfile
                .manifest
                .version_triple
                .unwrap_or(VersionTriple::default()),
            block_descriptions,
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

        writer.seek(SeekFrom::Start(end))?;
        Ok(())
    }
}
