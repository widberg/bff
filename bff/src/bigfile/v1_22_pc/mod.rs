use std::cmp::max;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{binread, binrw, parser, BinRead, BinResult, BinWrite, Endian};

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::ResourceData::{Data, SplitData};
use crate::bigfile::BigFile;
use crate::helpers::{write_align_to, DynArray};
use crate::names::NameType::{BlackSheep32, Kalisto32};
use crate::names::{Name, NameType};
use crate::bigfile::platforms::Platform;
use crate::traits::BigFileIo;
use crate::bigfile::versions::{Version, VersionTriple, VersionXple};
use crate::BffResult;

#[binrw]
#[derive(Debug)]
pub struct Resource<const S: u32 = 12> {
    #[br(temp)]
    #[bw(calc = data.len() as u32 + S)]
    data_size: u32,
    class_name: Name,
    pub name: Name,
    #[br(count = data_size - S)]
    data: Vec<u8>,
}

impl<const S: u32> Resource<S> {
    pub fn dump_resource<W: Write + Seek>(
        resource: &crate::bigfile::resource::Resource,
        writer: &mut W,
        endian: Endian,
    ) -> BinResult<()> {
        match &resource.data {
            Data(data) => {
                (data.len() as u32 + 12).write_options(writer, endian, ())?;
                resource.class_name.write_options(writer, endian, ())?;
                resource.name.write_options(writer, endian, ())?;
                data.write_options(writer, endian, ())?;
            }
            SplitData { link_header, body } => {
                let data_len = link_header.len() as u32 + body.len() as u32 + 12;
                data_len.write_options(writer, endian, ())?;
                resource.class_name.write_options(writer, endian, ())?;
                resource.name.write_options(writer, endian, ())?;
                link_header.write_options(writer, endian, ())?;
                body.write_options(writer, endian, ())?;
            }
        }

        Ok(())
    }

    pub fn read_resource<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
    ) -> BinResult<crate::bigfile::resource::Resource> {
        Ok(Self::read_options(reader, endian, ())?.into())
    }
}

impl<const S: u32> From<Resource<S>> for crate::bigfile::resource::Resource {
    fn from(resource: Resource<S>) -> crate::bigfile::resource::Resource {
        crate::bigfile::resource::Resource {
            class_name: resource.class_name,
            name: resource.name,
            link_name: None,
            compress: false,
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

    while reader.stream_position()? != end {
        blocks.push(Block::read_options(reader, endian, (block_size,))?);
    }

    Ok(blocks)
}

#[binread]
#[derive(Debug)]
#[br(import(version: Version, platform: Platform))]
pub struct BigFileV1_22PC<const HAS_VERSION_TRIPLE: bool = true, const KALISTO: bool = true> {
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
pub type BigFileV1_22PCNoVersionTripleBlackSheep = BigFileV1_22PC<false, false>;

impl<const HAS_VERSION_TRIPLE: bool, const KALISTO: bool>
    From<BigFileV1_22PC<HAS_VERSION_TRIPLE, KALISTO>> for BigFile
{
    fn from(bigfile: BigFileV1_22PC<HAS_VERSION_TRIPLE, KALISTO>) -> BigFile {
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
                compressed: None,
                objects,
            });
        }

        BigFile {
            manifest: Manifest {
                version: bigfile.version,
                version_xple: bigfile.version_triple.map(|x| x.into()),
                platform: bigfile.platform,
                bigfile_type: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            objects: resources,
        }
    }
}

impl<const HAS_VERSION_TRIPLE: bool, const KALISTO: bool> BigFileIo
    for BigFileV1_22PC<HAS_VERSION_TRIPLE, KALISTO>
{
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

    fn write<W: Write + Seek>(
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        // Remember starting position for writing block size
        let begin = writer.stream_position()?;

        let padding = [0xCD; 2048 - 256];
        writer.write_all(&padding)?;

        let mut block_size = 0u32;

        for block in bigfile.manifest.blocks.iter() {
            let block_begin = writer.stream_position()?;

            (block.objects.len() as u32).write_options(writer, endian, ())?;

            for resource in block.objects.iter() {
                let resource = bigfile.objects.get(&resource.name).unwrap();
                Resource::<12>::dump_resource(resource, writer, endian)?;
            }

            write_align_to(writer, 0x20000, 0xCD)?;

            let padding_end = writer.stream_position()?;
            block_size = max(block_size, (padding_end - block_begin) as u32);
        }

        // Write block size at the beginning of the file and restore position
        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;
        block_size.write_options(writer, endian, ())?;
        if HAS_VERSION_TRIPLE {
            match bigfile.manifest.version_xple.unwrap_or((0, 0, 0).into()) {
                VersionXple::Oneple(x) => (x, 0, 0),
                VersionXple::Triple(x) => x,
            }
            .write_options(writer, endian, ())?;
        }

        if let Some(tag) = tag {
            // TODO: Make sure the tag fits
            writer.write_all(tag.as_bytes())?;
        }

        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }

    const NAME_TYPE: NameType = if KALISTO { Kalisto32 } else { BlackSheep32 };

    type ResourceType = Resource;
}
