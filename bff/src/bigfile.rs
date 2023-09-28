use std::collections::HashMap;
use std::io::{Read, Seek};

use binrw::{BinRead, BinResult};
use serde::Serialize;

use crate::block::Block;
use crate::header::*;
use crate::manifest::*;
use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::pool::Pool;
use crate::BffResult;

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Object>,
}

#[binrw::parser(reader, endian)]
fn blocks_parser(
    block_descriptions: Vec<BlockDescription>,
    objects: &mut HashMap<Name, Object>,
) -> BinResult<Vec<ManifestBlock>> {
    let mut blocks: Vec<ManifestBlock> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        let mut block = Block::read_options(reader, endian, (&block_description,))?;
        let mut block_objects = Vec::with_capacity(block.objects.len());
        for object in block.objects.drain(..) {
            block_objects.push(ManifestObject {
                name: object.name(),
                compress: object.compress(),
            });

            objects.insert(object.name(), object);
        }

        blocks.push(ManifestBlock {
            offset: block_description.working_buffer_offset(),
            objects: block_objects,
        });
    }

    Ok(blocks)
}

#[binrw::parser(reader, endian)]
fn pool_parser(objects: &mut HashMap<Name, Object>) -> BinResult<ManifestPool> {
    let mut pool = Pool::read_options(reader, endian, ())?;

    let object_entry_indices = pool.header.object_descriptions_indices.into();
    let object_entries = pool
        .header
        .object_descriptions
        .iter()
        .map(|x| ManifestPoolObjectEntry {
            name: x.name(),
            reference_record_index: x.reference_records_index(),
        })
        .collect::<Vec<_>>();
    let reference_records = pool
        .header
        .reference_records
        .iter()
        .map(|x| ManifestPoolReferenceRecord {
            object_entries_starting_index: x.objects_name_starting_index(),
            object_entries_count: x.objects_name_count(),
        })
        .collect::<Vec<_>>();

    for pool_object in pool.objects.drain(..) {
        let name = pool_object.object.name();
        objects.get_mut(&name).unwrap().body = pool_object.object.body;
    }

    Ok(ManifestPool {
        object_entry_indices,
        object_entries,
        reference_records,
    })
}

impl BigFile {
    pub fn read_platform<R: Read + Seek>(reader: &mut R, platform: Platform) -> BffResult<Self> {
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
                version: header.version_string,
                version_triple: header.version_triple,
                platform,
                rtc: header.is_rtc,
                pool_manifest_unused: header.pool_manifest_unused,
                incredi_builder_string: header.incredi_builder_string,
                blocks,
                pool,
            },
            objects,
        })
    }
}
