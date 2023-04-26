use std::io::{Read, Seek};

use binread::*;
use serde::Serialize;
use crate::header::*;

#[derive(BinRead, Serialize, Debug)]
pub struct BigFile {
    header: Header,
}

impl BigFile {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        reader.read_le()
    }
}
