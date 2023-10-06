use std::cmp::max;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{binread, binrw, parser, BinRead, BinResult, BinWrite, Endian};

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::ResourceData;
use crate::bigfile::resource::ResourceData::Data;
use crate::bigfile::BigFile;
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

impl From<Resource> for crate::bigfile::resource::Resource {
    fn from(resource: Resource) -> crate::bigfile::resource::Resource {
        crate::bigfile::resource::Resource {
            class_name: resource.class_name,
            name: resource.name,
            data: Data(resource.data),
        }
    }
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
pub struct BigFileV1_22PC<const HAS_VERSION_TRIPLE: bool = true> {
    #[br(calc = version)]
    version: Version,
    #[br(calc = platform)]
    platform: Platform,
    #[br(temp)]
    block_size: u32,
    #[br(if(HAS_VERSION_TRIPLE))]
    pub version_triple: Option<VersionTriple>,
    #[br(align_before = 2048, parse_with = parse_blocks, args(block_size))]
    blocks: Vec<Block>,
}

pub type BigFileV1_22PCNoVersionTriple = BigFileV1_22PC<false>;

impl<const HAS_VERSION_TRIPLE: bool> From<BigFileV1_22PC<HAS_VERSION_TRIPLE>> for BigFile {
    fn from(bigfile: BigFileV1_22PC<HAS_VERSION_TRIPLE>) -> BigFile {
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
                resources.insert(resource.name, resource.into());
            }

            blocks.push(crate::bigfile::manifest::ManifestBlock {
                offset: None,
                checksum: None,
                objects,
            });
        }

        BigFile {
            manifest: Manifest {
                version: bigfile.version,
                version_triple: bigfile.version_triple,
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

impl<const HAS_VERSION_TRIPLE: bool> BigFileRead for BigFileV1_22PC<HAS_VERSION_TRIPLE> {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let bigfile: BigFileV1_22PC<HAS_VERSION_TRIPLE> =
            BigFileV1_22PC::read_options(reader, endian, (version, platform))?;
        Ok(bigfile.into())
    }
}

impl<const HAS_VERSION_TRIPLE: bool> BigFileWrite for BigFileV1_22PC<HAS_VERSION_TRIPLE> {
    fn write<W: Write + Seek>(bigfile: &BigFile, writer: &mut W) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        // Remember starting position for writing block size
        let begin = writer.stream_position()?;
        let mut block_size = 0u32;

        for block in bigfile.manifest.blocks.iter() {
            let block_begin = writer.stream_position()?;

            (block.objects.len() as u32).write_options(writer, endian, ())?;

            for resource in block.objects.iter() {
                let resource = bigfile.objects.get(&resource.name).unwrap();
                match resource.data {
                    Data(ref data) | ResourceData::CompressibleData { ref data, .. } => {
                        (data.len() as u32 + 12).write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        data.write_options(writer, endian, ())?;
                    }
                    ResourceData::ExtendedData {
                        ref link_header,
                        ref body,
                        ..
                    } => {
                        let data_len = link_header.len() as u32 + body.len() as u32 + 12;
                        data_len.write_options(writer, endian, ())?;
                        resource.class_name.write_options(writer, endian, ())?;
                        resource.name.write_options(writer, endian, ())?;
                        link_header.write_options(writer, endian, ())?;
                        body.write_options(writer, endian, ())?;
                    }
                };
            }

            let block_end = writer.stream_position()?;
            block_size = max(block_size, (block_end - block_begin) as u32);
        }

        // Write block size at the beginning of the file and restore position
        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;
        block_size.write_options(writer, endian, ())?;
        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }
}
