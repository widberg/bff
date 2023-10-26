use std::io::{Read, Seek, SeekFrom, Write};

use binrw::BinResult;

mod dynarray;
mod keyframer;
mod link_header;
mod map;
mod math;
mod option;
mod strings;

pub fn calculate_padding(position: usize, alignment: usize) -> usize {
    let remainder = position % alignment;
    if remainder != 0 {
        alignment - remainder
    } else {
        0
    }
}

pub fn calculated_padded(position: usize, alignment: usize) -> usize {
    position + calculate_padding(position, alignment)
}

pub fn write_align_to<W: Write + Seek>(
    writer: &mut W,
    alignment: usize,
    value: u8,
) -> BinResult<usize> {
    let padding = calculate_padding(writer.stream_position()? as usize, alignment);
    writer.write_all(&vec![value; padding])?;
    Ok(padding)
}

pub fn read_align_to<R: Read + Seek>(reader: &mut R, alignment: usize) -> BinResult<()> {
    let padding = calculate_padding(reader.stream_position()? as usize, alignment);
    reader.seek(SeekFrom::Current(padding as i64))?;
    Ok(())
}
pub use dynarray::*;
pub use keyframer::*;
pub use link_header::*;
pub use map::*;
pub use math::*;
pub use option::*;
pub use strings::*;
