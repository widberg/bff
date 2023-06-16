use crate::{block::Block, header::*, pool::Pool};
use binrw::*;
use serde::Serialize;

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
    pool: Pool,
}
