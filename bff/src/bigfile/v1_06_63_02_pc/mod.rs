pub mod block;
pub mod header;
pub mod object;
pub mod pool;

use std::collections::HashMap;
use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinResult};
use block::Block;
use header::*;
use pool::Pool;

use crate::bigfile::manifest::*;
use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::ExtendedData;
use crate::bigfile::BigFile;
use crate::names::Name;
use crate::platforms::Platform;
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

pub fn read_version_platform<R: Read + Seek>(
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
    let len = reader.seek(std::io::SeekFrom::End(0)).unwrap();
    assert_eq!(pos, len);

    Ok(BigFile {
        manifest: Manifest {
            version,
            version_triple: header.version_triple,
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

pub fn write<W: Write + Seek>(_bigfile: &BigFile, _writer: &W) -> BffResult<()> {
    todo!()
}
