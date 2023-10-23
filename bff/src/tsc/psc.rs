use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use itertools::Itertools;

use crate::helpers::StringUntilNull;
use crate::lz::{
    lz4_compress_data_with_header_writer_internal,
    lz4_decompress_data_with_header_parser_internal,
};
use crate::BffResult;

#[derive(Debug, Default)]
pub struct Psc {
    pub tscs: HashMap<PathBuf, String>,
}

impl ReadEndian for Psc {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl WriteEndian for Psc {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl BinRead for Psc {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let begin = reader.stream_position()?;
        let end = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(begin))?;

        let mut psc = Self::default();

        while reader.stream_position()? != end {
            let path_string = StringUntilNull::read(reader)?.0;
            let path = PathBuf::from(path_string);
            let cr = u8::read_le(reader)?;
            assert_eq!(cr, 0x0D);
            let lf = u8::read_le(reader)?;
            assert_eq!(lf, 0x0A);
            let data = StringUntilNull::read(reader)?.0;
            psc.tscs.insert(path, data);
        }

        Ok(psc)
    }
}

impl BinWrite for Psc {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        // TODO: In the game these are sorted by path
        // Root files first then subdirectories recursively in lexicographical order
        for (path, data) in self.tscs.iter() {
            writer.write_all(
                path.components()
                    .map(|x| x.as_os_str().to_str().unwrap())
                    .join("\\")
                    .as_bytes(),
            )?;
            writer.write_all(&[0x00, 0x0D, 0x0A])?;
            writer.write_all(data.as_bytes())?;
            writer.write_all(&[0x00])?;
        }

        Ok(())
    }
}

impl Psc {
    pub fn read<R: Read + Seek>(reader: &mut R) -> BffResult<Self> {
        let mut psc_data = Cursor::new(lz4_decompress_data_with_header_parser_internal(
            reader,
            Endian::Little,
            (),
        )?);

        Ok(<Self as BinRead>::read(&mut psc_data)?)
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BffResult<()> {
        let mut psc_data = Cursor::new(Vec::new());
        <Self as BinWrite>::write(self, &mut psc_data)?;
        lz4_compress_data_with_header_writer_internal(
            &psc_data.into_inner(),
            writer,
            Endian::Little,
            (),
        )?;
        Ok(())
    }
}