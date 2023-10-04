use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{args, binread, binrw, parser, BinRead, BinResult, BinWrite, Endian};

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::ResourceData::Data;
use crate::dynarray::DynArray;
use crate::names::Name;
use crate::platforms::Platform;
use crate::traits::{BigFileRead, BigFileWrite};
use crate::versions::{Version, VersionTriple};
use crate::BffResult;

#[binrw]
#[derive(Debug)]
pub struct Resource {
    #[br(temp)]
    #[bw(calc = data.len() as u32 + 12)]
    data_size: u32,
    class_name: Name,
    name: Name,
    #[br(count = data_size - 12)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug)]
#[br(import(block_size: u32), stream = s)]
pub struct Block {
    #[br(temp, try_calc = s.stream_position())]
    begin: u64,
    pub resources: DynArray<Resource>,
    #[br(temp, try_calc = s.stream_position())]
    end: u64,
    #[br(temp, pad_after = block_size as u64 - (end - begin))]
    _padding: (),
}

impl BinWrite for Block {
    type Args<'a> = (u32,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        (block_size,): Self::Args<'_>,
    ) -> BinResult<()> {
        let begin = writer.stream_position()?;
        self.resources.write_options(writer, endian, ())?;
        let end = writer.stream_position()?;
        vec![0u8; block_size as usize - (end - begin) as usize].write_be(writer)?;
        Ok(())
    }
}

#[parser(reader, endian)]
fn parse_blocks(block_size: u32) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    let begin = reader.stream_position()?;
    let end = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(begin))?;

    loop {
        let begin = reader.stream_position()?;

        if begin == end {
            break;
        }

        blocks.push(Block::read_options(reader, endian, (block_size,))?);
    }

    Ok(blocks)
}

#[binread]
#[derive(Debug)]
#[br(import(version: Version, platform: Platform))]
pub struct BigFile {
    #[br(calc = version)]
    version: Version,
    #[br(calc = platform)]
    platform: Platform,
    #[br(temp)]
    block_size: u32,
    #[br(align_after = 2048)]
    pub version_triple: VersionTriple,
    #[br(parse_with = parse_blocks, args(block_size))]
    blocks: Vec<Block>,
}

impl From<BigFile> for crate::bigfile::BigFile {
    fn from(bigfile: BigFile) -> crate::bigfile::BigFile {
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
                checksum: None,
                objects,
            });
        }

        crate::bigfile::BigFile {
            manifest: Manifest {
                version: bigfile.version,
                version_triple: Some(bigfile.version_triple),
                platform: bigfile.platform,
                rtc: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            objects: resources,
        }
    }
}

impl BigFileRead for BigFile {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<crate::bigfile::BigFile> {
        let endian = platform.into();
        let bigfile = BigFile::read_options(reader, endian, (version, platform))?;
        Ok(bigfile.into())
    }
}

impl BigFileWrite for BigFile {
    fn write<W: Write + Seek>(
        _bigfile: &crate::bigfile::BigFile,
        _writer: &mut W,
    ) -> BffResult<()> {
        todo!()
    }
}
