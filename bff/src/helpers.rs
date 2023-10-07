use std::io::{Seek, Write};

use binrw::{writer, BinResult};

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
