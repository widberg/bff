use std::io::{Read, Seek};

use binrw::*;
use serde::Serialize;

use crate::block::Block;
use crate::header::*;
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
    #[br(parse_with = blocks_parser, args(header.block_descriptions()))]
    blocks: Vec<Block>,
    #[br(if(header.has_pool()))]
    pool: Option<Pool>,
}

impl BigFile {
    pub fn read_endian<R: Read + Seek>(reader: &mut R, endian: Endian) -> BffResult<Self> {
        Ok(Self::read_options(reader, endian, ())?)
    }
}
