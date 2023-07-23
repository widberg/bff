use std::io::{Read, Seek, SeekFrom};

use binrw::*;
use serde::Serialize;

use crate::block::Block;
use crate::header::*;
use crate::platforms::Platform;
use crate::pool::Pool;
use crate::BffResult;

#[binrw::parser(reader, endian)]
fn blocks_parser(block_descriptions: &Vec<BlockDescription>) -> BinResult<Vec<Block>> {
    let mut blocks: Vec<Block> = Vec::with_capacity(block_descriptions.len());

    for block_description in block_descriptions {
        blocks.push(Block::read_options(reader, endian, (block_description,))?)
    }

    Ok(blocks)
}

#[derive(BinRead, Serialize, Debug)]
pub struct BigFile {
    header: Header,
    #[br(if(header.pool_offset().is_some()), seek_before = SeekFrom::Start(header.pool_offset().unwrap() as u64), restore_position)]
    pool: Option<Pool>,
    #[br(parse_with = blocks_parser, args(header.block_descriptions()))]
    blocks: Vec<Block>,
}

impl BigFile {
    pub fn read_endian<R: Read + Seek>(reader: &mut R, endian: Endian) -> BffResult<Self> {
        Ok(Self::read_options(reader, endian, ())?)
    }

    pub fn read_platform<R: Read + Seek>(reader: &mut R, platform: Platform) -> BffResult<Self> {
        Ok(Self::read_options(reader, platform.into(), ())?)
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}
