use binrw::{writer, BinRead, BinResult, BinWrite};
use serde::Serialize;

use crate::header::BlockDescription;
use crate::object::Object;

#[writer(writer, endian)]
fn write_align_after<'a, T: BinWrite<Args<'a> = ()>>(
    value: &T,
    align: u64,
    fill: u8,
) -> BinResult<()> {
    value.write_options(writer, endian, ())?;
    let pos = writer.stream_position()?;
    let padding = align - (pos % align);
    vec![fill; padding as usize].write_options(writer, endian, ())
}

#[derive(BinRead, BinWrite, Serialize, Debug)]
#[br(import(block_description: &BlockDescription))]
pub struct Block {
    #[br(count = block_description.object_count, align_after = 2048)]
    #[bw(write_with = write_align_after, args(2048, 0xFF))]
    pub objects: Vec<Object>,
}
