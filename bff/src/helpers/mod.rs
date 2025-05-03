use std::io::{self, Read, Seek, SeekFrom, Write};

use binrw::BinResult;

mod dynarray;
mod keyframer;
mod link_header;
mod map;
mod math;
mod option;
mod strings;

pub fn calculate_padding(position: usize, alignment: usize) -> usize {
    position.next_multiple_of(alignment) - position
}

pub fn calculated_padded(position: usize, alignment: usize) -> usize {
    position.next_multiple_of(alignment)
}

pub fn copy_repeat<W: Write>(writer: &mut W, value: u8, length: u64) -> io::Result<u64> {
    std::io::copy(&mut std::io::repeat(value).take(length), writer)
}

pub fn write_align_to<W: Write + Seek>(
    writer: &mut W,
    alignment: usize,
    value: u8,
) -> BinResult<usize> {
    let padding = calculate_padding(writer.stream_position()? as usize, alignment) as u64;
    Ok(copy_repeat(writer, value, padding)? as usize)
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
