use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, Write};

use binrw::{binread, parser, BinRead, BinResult, BinWrite, Endian, args};

use super::v1_22_pc::Resource;
use crate::bigfile::manifest::Manifest;
use crate::bigfile::BigFile;
use crate::helpers::{read_align_to, DynArray};
use crate::lz::lzo_decompress;
use crate::names::NameType;
use crate::names::NameType::BlackSheep32;
use crate::platforms::Platform;
use crate::traits::BigFileIo;
use crate::versions::Version;
use crate::BffResult;

#[derive(Debug)]
pub struct Block {
    pub compressed: bool,
    pub checksum: Option<u32>,
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

#[parser(reader, endian)]
fn parse_blocks<const MQFEL: bool>(
    decompressed_block_size: u32,
    block_sizes: Vec<u32>,
) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    for block_size in block_sizes {
        let checksum = if MQFEL {
            Some(u32::read_options(reader, endian, ())?)
        } else {
            None
        };
        let resource_count = u32::read_options(reader, endian, ())?;

        if block_size != decompressed_block_size {
            let block_size = if MQFEL { block_size - 8 } else { block_size };

            let mut compressed = vec![0; block_size as usize];
            reader.read_exact(&mut compressed)?;
            let decompressed =
                lzo_decompress(&compressed, decompressed_block_size as usize).unwrap();
            let mut decompressed = Cursor::new(decompressed);
            blocks.push(Block {
                compressed: true,
                checksum,
                resources: Vec::<Resource>::read_options(&mut decompressed, endian, args! { count: resource_count as usize })?
            });
        } else {
            blocks.push(Block {
                compressed: false,
                checksum,
                resources: Vec::<Resource>::read_options(reader, endian, args! { count: resource_count as usize })?
            });
        }
        read_align_to(reader, 2048)?;
    }

    Ok(blocks)
}

#[derive(Debug, BinRead)]
#[br(repr = u32)]
enum CompressionType {
    None,
    Lzo,
}

#[binread]
#[derive(Debug)]
#[br(import(version: Version, platform: Platform))]
pub struct BigFileV2_07PC<const MQFEL: bool = false> {
    #[br(calc = version)]
    version: Version,
    #[br(calc = platform)]
    platform: Platform,
    #[br(temp)]
    decompressed_block_size: u32,
    #[br(temp)]
    _compression_type: CompressionType,
    #[br(temp)]
    block_sizes: DynArray<u32>,
    #[br(align_before = 2048, parse_with = parse_blocks::<MQFEL, _>, args(decompressed_block_size, block_sizes.inner))]
    blocks: Vec<Block>,
}

pub type BigFileV2_07PCMQFEL = BigFileV2_07PC<true>;

impl<const MQFEL: bool> From<BigFileV2_07PC<MQFEL>> for BigFile {
    fn from(bigfile: BigFileV2_07PC<MQFEL>) -> BigFile {
        let mut blocks = Vec::with_capacity(bigfile.blocks.len());
        let mut resources = HashMap::new();

        for block in bigfile.blocks.into_iter() {
            let mut objects = Vec::with_capacity(block.resources.len());

            for resource in block.resources {
                objects.push(crate::bigfile::manifest::ManifestObject {
                    name: resource.name,
                    compress: None,
                });
                resources.insert(resource.name, resource.into());
            }

            blocks.push(crate::bigfile::manifest::ManifestBlock {
                offset: None,
                checksum: None,
                compressed: Some(block.compressed),
                objects,
            });
        }

        BigFile {
            manifest: Manifest {
                version: bigfile.version,
                version_triple: None,
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

impl<const MQFEL: bool> BigFileIo for BigFileV2_07PC<MQFEL> {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let bigfile: BigFileV2_07PC<MQFEL> =
            BigFileV2_07PC::read_options(reader, endian, (version, platform))?;
        Ok(bigfile.into())
    }

    fn write<W: Write + Seek>(
        _bigfile: &BigFile,
        _writer: &mut W,
        _tag: Option<&str>,
    ) -> BffResult<()> {
        todo!()
    }

    const NAME_TYPE: NameType = BlackSheep32;

    type ResourceType = Resource;
}
