use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use binrw::{BinRead, BinResult, BinWrite, Endian, NullString, args};
use itertools::Itertools;

use crate::BffResult;
use crate::helpers::copy_repeat;
use crate::lz::{lzo_compress, lzo_decompress};

#[derive(Debug, Default)]
pub struct Cps {
    pub tscs: HashMap<PathBuf, String>,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
enum Param {
    #[brw(magic = 0u8)]
    String(NullString),
    #[brw(magic = 1u8)]
    Float(f32),
}

#[binrw::parser(reader, endian)]
fn decode_cps_script() -> BinResult<String> {
    let mut script = String::new();

    let num_lines = u32::read_options(reader, endian, ())?;

    for _ in 0..num_lines {
        let num_params = u8::read_options(reader, endian, ())?;
        if num_params != 0 {
            let command_name = i32::read_options(reader, endian, ())?;
            script.push_str(command_name.to_string().as_str());
            for _ in 1..num_params {
                script.push(' ');
                let param = Param::read_options(reader, endian, ())?;
                match param {
                    Param::String(s) => script.push_str(format!(r#""{}""#, s).as_str()),
                    Param::Float(f) => script.push_str(f.to_string().as_str()),
                }
            }
        }
        script.push('\n');
    }

    Ok(script)
}

#[binrw::writer(writer, endian)]
fn encode_cps_script(script: &str) -> BinResult<()> {
    let start = writer.stream_position()?;
    let mut num_lines: u32 = 0;
    num_lines.write_options(writer, endian, ())?;

    for line in script.lines() {
        let mut chars = line.chars().peekable();
        chars
            .peeking_take_while(|c| c.is_whitespace())
            .for_each(drop);
        let Ok(command_name) = (&mut chars)
            .take_while(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<i32>()
        else {
            0u8.write_options(writer, endian, ())?;
            continue;
        };
        let mut params: Vec<Param> = Vec::new();
        loop {
            chars
                .peeking_take_while(|c| c.is_whitespace())
                .for_each(drop);
            let next = chars.peek();
            match next {
                Some(&'"') => {
                    chars.next();
                    let param = (&mut chars).take_while(|c| *c != '"').collect::<String>();
                    params.push(Param::String(param.into()));
                }
                Some(_) => {
                    let param = (&mut chars)
                        .take_while(|c| !c.is_whitespace())
                        .collect::<String>()
                        .parse::<f32>()
                        .unwrap();
                    params.push(Param::Float(param));
                }
                None => break,
            }
        }

        let num_params = 1 + params.len() as u8;
        num_params.write_options(writer, endian, ())?;
        command_name.write_options(writer, endian, ())?;
        for param in params {
            param.write_options(writer, endian, ())?;
        }

        num_lines += 1;
    }

    let end = writer.stream_position()?;
    writer.seek(SeekFrom::Start(start))?;
    num_lines.write_options(writer, endian, ())?;
    writer.seek(SeekFrom::Start(end))?;

    Ok(())
}

impl BinRead for Cps {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut cps = Self::default();

        let _version = <[u8; 8]>::read_options(reader, endian, ())?;
        let _flags = u32::read_options(reader, endian, ())?;
        let script_count = u32::read_options(reader, endian, ())?;

        for _ in 0..script_count {
            let name = i32::read_options(reader, endian, ())?;
            let uncompressed_size = u32::read_options(reader, endian, ())?;
            let compressed_size = u32::read_options(reader, endian, ())?;
            let offset_in_bigfile = u32::read_options(reader, endian, ())?;

            let pos = reader.stream_position()?;
            reader.seek(SeekFrom::Start(offset_in_bigfile as u64))?;
            let compressed_data =
                Vec::<u8>::read_args(reader, args! { count: compressed_size as usize })?;
            reader.seek(SeekFrom::Start(pos))?;

            let uncompressed_data =
                lzo_decompress(&compressed_data, uncompressed_size as usize).unwrap();
            let decoded_data = decode_cps_script(&mut Cursor::new(uncompressed_data), endian, ())?;

            let path = PathBuf::from(format!("{}.tsc", name));
            cps.tscs.insert(path, decoded_data);
        }

        Ok(cps)
    }
}

impl BinWrite for Cps {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let version = CPS_VERSION;
        let flags = 0u32;
        let script_count = self.tscs.len() as u32;
        version.write_options(writer, endian, ())?;
        flags.write_options(writer, endian, ())?;
        script_count.write_options(writer, endian, ())?;

        let pos = writer.stream_position()?;
        // Zero fill the space for the script definitions
        copy_repeat(writer, 0, 0x10 * script_count as u64)?;
        writer.seek(SeekFrom::Start(pos))?;

        for (path, script) in self.tscs.iter() {
            let name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let mut encoded_data = Cursor::new(Vec::new());
            encode_cps_script(script, &mut encoded_data, endian, ())?;
            let encoded_data = encoded_data.into_inner();
            let uncompressed_size = encoded_data.len() as u32;
            let mut compressed_data = Cursor::new(Vec::new());
            lzo_compress(&encoded_data, &mut compressed_data).unwrap();
            let compressed_data = compressed_data.into_inner();
            let compressed_size = compressed_data.len() as u32;

            name.write_options(writer, endian, ())?;
            uncompressed_size.write_options(writer, endian, ())?;
            compressed_size.write_options(writer, endian, ())?;
            let pos = writer.stream_position()?;
            writer.seek(SeekFrom::End(0))?;
            let offset_in_bigfile = writer.stream_position()? as u32;
            compressed_data.write(writer)?;
            writer.seek(SeekFrom::Start(pos))?;
            offset_in_bigfile.write_options(writer, endian, ())?;
        }

        Ok(())
    }
}

const CPS_VERSION: &[u8; 8] = b"OPAL_1.0";
const CPS_FIRST_CHAR: u8 = b'O';
const CPS_SEED_STEP: u8 = 37;

#[allow(clippy::unbuffered_bytes)]
#[binrw::parser(reader)]
fn cps_crypt() -> BinResult<Vec<u8>> {
    let mut data = Vec::new();

    let mut seed = CPS_FIRST_CHAR;
    for byte in reader.bytes() {
        let byte = byte?;
        data.push(byte ^ seed);
        seed = seed.wrapping_add(CPS_SEED_STEP);
    }

    Ok(data)
}

impl Cps {
    pub fn read<R: Read + Seek>(reader: &mut R, endian: Endian) -> BffResult<Self> {
        let first_char = u8::read_options(reader, endian, ())?;
        reader.seek(SeekFrom::Start(0))?;
        if first_char == CPS_FIRST_CHAR {
            Ok(<Self as BinRead>::read_options(reader, endian, ())?)
        } else {
            let mut cps_data = Cursor::new(cps_crypt(reader, endian, ())?);
            Ok(<Self as BinRead>::read_options(&mut cps_data, endian, ())?)
        }
    }

    pub fn write<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        unencrypted: bool,
    ) -> BffResult<()> {
        if unencrypted {
            <Self as BinWrite>::write_options(self, writer, endian, ())?;
        } else {
            let mut cps_data = Cursor::new(Vec::new());
            <Self as BinWrite>::write_options(self, &mut cps_data, endian, ())?;
            cps_data.seek(SeekFrom::Start(0))?;
            let encrypted_data = cps_crypt(&mut cps_data, endian, ())?;
            writer.write_all(&encrypted_data)?;
        }

        Ok(())
    }
}
