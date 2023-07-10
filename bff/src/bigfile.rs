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
    #[br(if(header.has_pool()))]
    pool: Option<Pool>,
}

#[cfg(test)]
mod tests {
    use crate::platforms::extension_to_endian;

    use super::*;
    use binrw::io::BufReader;
    use std::{fs::File, path::PathBuf};
    use test_generator::test_resources;

    #[test_resources("data/bigfiles/**/*.*")]
    fn read(bigfile_path_str: &str) {
        let mut bigfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bigfile_path.pop();
        bigfile_path.push(bigfile_path_str);
        let endian = match bigfile_path.extension() {
            Some(extension) => extension_to_endian(extension).unwrap_or(Endian::Little),
            None => Endian::Little,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let _ = BigFile::read_options(&mut reader, endian, ()).unwrap();
    }
}
