use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{args, binread, BinRead, BinResult, BinWrite, parser};

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::ResourceData::Data;
use crate::dynarray::DynArray;
use crate::names::Name;
use crate::platforms::Platform;
use crate::versions::{Version, VersionTriple};
use crate::BffResult;

#[derive(Debug, BinRead, BinWrite)]
pub struct Resource {
    data_size: u32,
    class_name: Name,
    name: Name,
    #[br(count = data_size - 12)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug)]
#[br(import { block_size: u32 }, stream = s)]
pub struct Block {
    #[br(temp, try_calc = s.stream_position())]
    begin: u64,
    pub resources: DynArray<Resource>,
    #[br(temp, try_calc = s.stream_position())]
    end: u64,
    #[br(temp, pad_after = block_size - (end - begin) as u32)]
    _padding: (),
}

#[parser(reader, endian)]
fn parse_blocks(block_size: u32) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    loop {
        let begin = reader.stream_position()?;
        let block = Block::read_options(reader, endian, args! { block_size });
        match block {
            Ok(block) => blocks.push(block),
            Err(_) => {
                let end = reader.seek(SeekFrom::End(0))?;
                assert_eq!(begin, end);
                break;
            },
        }
    }

    Ok(blocks)
}

#[derive(Debug, BinRead)]
pub struct Header {
    pub block_size: u32,
    #[br(align_after = 2048)]
    pub version_triple: VersionTriple,
}

#[derive(Debug, BinRead)]
pub struct BigFile {
    pub header: Header,
    #[br(parse_with = parse_blocks, args(header.block_size))]
    blocks: Vec<Block>,
}

pub fn read_version_platform<R: Read + Seek>(
    reader: &mut R,
    version: Version,
    platform: Platform,
) -> BffResult<crate::bigfile::BigFile> {
    let bigfile = BigFile::read_options(reader, platform.into(), ())?;

    let mut blocks = Vec::with_capacity(bigfile.blocks.len());
    let mut resources = HashMap::new();

    for block in bigfile.blocks.into_iter() {
        let mut objects = Vec::with_capacity(block.resources.len());

        // Accessing the inner vector directly feels evil
        for resource in block.resources.inner.into_iter() {
            objects.push(crate::bigfile::manifest::ManifestObject {
                name: resource.name,
                compress: None,
            });
            resources.insert(
                resource.name,
                crate::bigfile::resource::Resource {
                    class_name: resource.class_name,
                    name: resource.name,
                    data: Data(resource.data),
                },
            );
        }

        blocks.push(crate::bigfile::manifest::ManifestBlock {
            offset: None,
            objects,
        });
    }

    Ok(crate::bigfile::BigFile {
        manifest: Manifest {
            version,
            version_triple: bigfile.header.version_triple,
            platform,
            rtc: None,
            pool_manifest_unused: None,
            incredi_builder_string: None,
            blocks,
            pool: None,
        },
        objects: resources,
    })
}

pub fn write<W: Write + Seek>(_bigfile: &crate::bigfile::BigFile, _writer: &W) -> BffResult<()> {
    todo!()
}
