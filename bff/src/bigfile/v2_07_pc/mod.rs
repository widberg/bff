use std::cmp::max;
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, args, binread, parser};

use super::v1_22_pc::Resource;
use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::Manifest;
use crate::bigfile::platforms::Platform;
use crate::bigfile::versions::Version;
use crate::helpers::{DynArray, calculated_padded, read_align_to, write_align_to};
use crate::lz::{lzo_compress, lzo_decompress};
use crate::names::NameType;
use crate::names::NameType::BlackSheep32;
use crate::traits::BigFileIo;

#[derive(Debug)]
pub struct Block {
    pub compressed: bool,
    pub resources: Vec<Resource>,
}

impl BinWrite for Block {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.resources.write_options(writer, endian, ())?;
        Ok(())
    }
}

const SHAUN_PROTO: usize = 0;
const SHAUN: usize = 1;

#[parser(reader, endian)]
fn parse_blocks<const GAME: usize>(
    decompressed_block_size: u32,
    block_sizes: &[u32],
) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    for block_size in block_sizes {
        let block_start = reader.stream_position()?;

        let resource_count = u32::read_options(reader, endian, ())?;

        if *block_size != decompressed_block_size {
            let mut compressed = vec![
                0;
                (*block_size
                    - match GAME {
                        SHAUN_PROTO => 0,
                        SHAUN => 4,
                        _ => unreachable!(),
                    }) as usize
            ];
            reader.read_exact(&mut compressed)?;
            let decompressed =
                lzo_decompress(&compressed, decompressed_block_size as usize).unwrap();
            let mut decompressed = Cursor::new(decompressed);
            blocks.push(Block {
                compressed: true,
                resources: Vec::<Resource>::read_options(
                    &mut decompressed,
                    endian,
                    args! { count: resource_count as usize },
                )?,
            });
            read_align_to(reader, 2048)?;
        } else {
            blocks.push(Block {
                compressed: false,
                resources: Vec::<Resource>::read_options(
                    reader,
                    endian,
                    args! { count: resource_count as usize },
                )?,
            });
            reader.seek(SeekFrom::Start(block_start + *block_size as u64))?;
        }
    }

    Ok(blocks)
}

#[derive(Debug, BinRead, BinWrite, Copy, Clone)]
#[brw(repr = u32)]
pub enum CompressionType {
    None,
    Lzo,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Header {
    pub decompressed_block_size: u32,
    pub compression_type: CompressionType,
    pub block_sizes: DynArray<u32>,
}

#[binread]
#[derive(Debug)]
#[br(import(version: Version, platform: Platform))]
pub struct BigFileV2_07PC<const GAME: usize> {
    #[br(calc = version)]
    version: Version,
    #[br(calc = platform)]
    platform: Platform,
    #[br(temp)]
    header: Header,
    #[br(align_before = 2048, parse_with = parse_blocks::<GAME, _>, args(header.decompressed_block_size, &header.block_sizes.inner))]
    blocks: Vec<Block>,
}

pub type BigFileV2_07PCSHAUN = BigFileV2_07PC<SHAUN>;
pub type BigFileV2_07PCPROTO = BigFileV2_07PC<SHAUN_PROTO>;

impl<const GAME: usize> From<BigFileV2_07PC<GAME>> for BigFile {
    fn from(bigfile: BigFileV2_07PC<GAME>) -> Self {
        let mut blocks = Vec::with_capacity(bigfile.blocks.len());
        let mut resources = HashMap::new();

        for block in bigfile.blocks.into_iter() {
            let mut block_resources = Vec::with_capacity(block.resources.len());

            for resource in block.resources {
                block_resources.push(crate::bigfile::manifest::ManifestResource {
                    name: resource.name,
                    compress: None,
                });
                resources.insert(resource.name, resource.into());
            }

            blocks.push(crate::bigfile::manifest::ManifestBlock {
                offset: None,
                checksum: None,
                compress: Some(block.compressed),
                resources: block_resources,
            });
        }

        Self {
            manifest: Manifest {
                version: bigfile.version,
                version_xple: None,
                platform: bigfile.platform,
                bigfile_type: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            resources,
        }
    }
}

impl<const GAME: usize> BigFileIo for BigFileV2_07PC<GAME> {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let bigfile = Self::read_options(reader, endian, (version, platform))?;
        Ok(bigfile.into())
    }

    fn write<W: Write + Seek>(
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        // Remember starting position for writing header
        let begin = writer.stream_position()?;
        writer.seek(SeekFrom::Current(0x700))?;

        let mut decompressed_block_size = 0;

        let mut blocks = Vec::new();

        for block in bigfile.manifest.blocks.iter() {
            let mut block_writer = Cursor::new(Vec::new());

            for resource in block.resources.iter() {
                let resource = bigfile.resources.get(&resource.name).unwrap();
                Resource::<12>::dump_resource(resource, &mut block_writer, endian)?;
            }

            let block_data = block_writer.into_inner();

            decompressed_block_size = max(decompressed_block_size, block_data.len() as u32);

            blocks.push((
                block.resources.len() as u32,
                block.checksum.unwrap_or(0),
                block.compress.unwrap_or(false),
                block_data,
            ));
        }

        decompressed_block_size = calculated_padded(decompressed_block_size as usize, 2048) as u32;
        let mut block_sizes = Vec::new();
        let mut compression_type = CompressionType::None;

        for (resource_count, _, compressed, mut block_data) in blocks {
            let block_begin = writer.stream_position()?;

            resource_count.write_options(writer, endian, ())?;

            block_data.resize(decompressed_block_size as usize, 0);

            if compressed {
                // FIXME: Compressed data isn't a 1-to-1 match but round trips correctly?
                // Some size is probably off somewhere. Or the version of minilzo is wrong.
                compression_type = CompressionType::Lzo;
                lzo_compress(&block_data, writer)?;
            } else {
                writer.write_all(&block_data)?;
            }

            let block_end = writer.stream_position()?;

            write_align_to(writer, 2048, 0)?;

            block_sizes.push(
                (block_end
                    - block_begin
                    - if compressed {
                        match GAME {
                            SHAUN_PROTO => 0,
                            SHAUN => 4,
                            _ => unreachable!(),
                        }
                    } else {
                        0
                    }) as u32,
            );
        }

        // Write header at the beginning of the file and restore position
        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;

        let header = Header {
            decompressed_block_size,
            compression_type,
            block_sizes: block_sizes.into(),
        };
        header.write_options(writer, endian, ())?;

        if let Some(tag) = tag {
            // TODO: Make sure the tag fits
            writer.write_all(tag.as_bytes())?;
        }

        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }

    const NAME_TYPE: NameType = BlackSheep32;

    // FIXME: I'm not convinced this is correct, see the other v2_X BF
    type ResourceType = Resource;
}
