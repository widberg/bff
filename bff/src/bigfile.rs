use std::io::{Read, Seek};

use binrw::*;
use serde::Serialize;
use crate::{header::*, block::Block};

#[derive(Serialize, Debug)]
pub struct BigFile {
    header: Header,
    blocks: Vec<Block>,
}

impl BigFile {
    pub fn read<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        reader.read_le()
    }
}

impl BinRead for BigFile {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let header = Header::read_options(reader, endian, ())?;
        
        // Pool
        Ok(BigFile {
            header
        })
    }
}
