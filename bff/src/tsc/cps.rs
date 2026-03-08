use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use binrw::{BinRead, BinResult, BinWrite, Endian, NullString, args};
use itertools::Itertools;

use crate::BffResult;
use crate::helpers::copy_repeat;
use crate::lz::{lzo_compress, lzo_decompress};
use crate::names::{Name, NameContext};

const DEFAULT_CPS_IN_NAMES: &str = include_str!("ALLSCRIPTS.CPSNameWii");

pub fn read_default_cps_names(name_context: &NameContext) -> BffResult<()> {
    let mut reader = crate::BufReader::new(Cursor::new(DEFAULT_CPS_IN_NAMES.as_bytes()));
    name_context.read(&mut reader)?;
    Ok(())
}

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

fn format_cps_float(value: f32) -> String {
    let mut s = value.to_string();

    // Normalize shorthand forms so output always has an explicit whole part.
    if s.starts_with('.') {
        s.insert(0, '0');
    } else if s.starts_with("-.") || s.starts_with("+.") {
        s.insert(1, '0');
    }

    // Ensure an explicit decimal part for non-exponent and exponent forms.
    let e_idx = s.find('e').or_else(|| s.find('E'));
    if let Some(i) = e_idx {
        if !s[..i].contains('.') {
            s.insert_str(i, ".0");
        }
    } else if !s.contains('.') {
        s.push_str(".0");
    }

    s.push('f');
    s
}

fn parse_cps_float_token(token: &str) -> Option<f32> {
    let token = token.trim();
    let token = token
        .strip_suffix('f')
        .or_else(|| token.strip_suffix('F'))
        .unwrap_or(token)
        .trim_end();

    if token.is_empty() {
        return None;
    }

    let normalized = if let Some(rest) = token.strip_prefix("-.") {
        format!("-0.{rest}")
    } else if let Some(rest) = token.strip_prefix("+.") {
        format!("+0.{rest}")
    } else if let Some(rest) = token.strip_prefix('.') {
        format!("0.{rest}")
    } else {
        token.to_string()
    };

    normalized.parse::<f32>().ok()
}

fn escape_cps_quoted_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(c),
        }
    }
    escaped
}

fn format_cps_string_param(value: &str) -> String {
    let needs_quotes = value.is_empty()
        || value.chars().any(|c| c.is_whitespace())
        || value.starts_with('"')
        || parse_cps_float_token(value).is_some();

    if needs_quotes {
        format!("\"{}\"", escape_cps_quoted_string(value))
    } else {
        value.to_string()
    }
}

fn decode_cps_script<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    name_context: &NameContext,
    missing_command_names: &mut Vec<Name>,
    missing_command_names_seen: &mut HashSet<Name>,
) -> BinResult<String> {
    let mut script = String::new();

    let num_lines = u32::read_options(reader, endian, ())?;

    for _ in 0..num_lines {
        let num_params = u8::read_options(reader, endian, ())?;
        if num_params != 0 {
            let command_name = name_context.scope(|| Name::read_options(reader, endian, ()))?;
            if name_context.resolve(&command_name).is_none()
                && missing_command_names_seen.insert(command_name)
            {
                missing_command_names.push(command_name);
            }
            script.push_str(command_name.with_context(name_context).to_string().as_str());
            for _ in 1..num_params {
                script.push(' ');
                let param = Param::read_options(reader, endian, ())?;
                match param {
                    Param::String(s) => script.push_str(format_cps_string_param(&s.to_string()).as_str()),
                    Param::Float(f) => script.push_str(format_cps_float(f).as_str()),
                }
            }
        }
        script.push('\n');
    }

    Ok(script)
}

fn encode_cps_script<W: Write + Seek>(
    script: &str,
    writer: &mut W,
    endian: Endian,
    name_context: &NameContext,
) -> BinResult<()> {
    let start = writer.stream_position()?;
    let mut num_lines: u32 = 0;
    num_lines.write_options(writer, endian, ())?;

    for line in script.lines() {
        let mut chars = line.chars().peekable();
        chars
            .peeking_take_while(|c| c.is_whitespace())
            .for_each(drop);
        let command_name_token = (&mut chars)
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if command_name_token.is_empty() {
            0u8.write_options(writer, endian, ())?;
            continue;
        }
        let command_name = name_context.parse_or_hash_name(&command_name_token);
        let mut params: Vec<Param> = Vec::new();
        loop {
            chars
                .peeking_take_while(|c| c.is_whitespace())
                .for_each(drop);
            let next = chars.peek();
            match next {
                Some(&'"') => {
                    chars.next();
                    let mut param = String::new();
                    let mut escaped = false;
                    for c in chars.by_ref() {
                        if escaped {
                            match c {
                                'n' => param.push('\n'),
                                'r' => param.push('\r'),
                                't' => param.push('\t'),
                                '"' => param.push('"'),
                                '\\' => param.push('\\'),
                                _ => param.push(c),
                            }
                            escaped = false;
                            continue;
                        }

                        match c {
                            '\\' => escaped = true,
                            '"' => break,
                            _ => param.push(c),
                        }
                    }
                    if escaped {
                        param.push('\\');
                    }
                    params.push(Param::String(param.into()));
                }
                Some(_) => {
                    let param_token = (&mut chars)
                        .take_while(|c| !c.is_whitespace())
                        .collect::<String>();
                    if let Some(param) = parse_cps_float_token(&param_token) {
                        params.push(Param::Float(param));
                    } else {
                        params.push(Param::String(param_token.into()));
                    }
                }
                None => break,
            }
        }

        let num_params = 1 + params.len() as u8;
        num_params.write_options(writer, endian, ())?;
        name_context.scope(|| command_name.write_options(writer, endian, ()))?;
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
    type Args<'a> = (&'a NameContext,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (name_context,): Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut cps = Self::default();
        let mut missing_command_names = Vec::new();
        let mut missing_command_names_seen = HashSet::new();
        let mut missing_file_names = Vec::new();
        let mut missing_file_names_seen = HashSet::new();

        let _version = <[u8; 8]>::read_options(reader, endian, ())?;
        let _flags = u32::read_options(reader, endian, ())?;
        let script_count = u32::read_options(reader, endian, ())?;

        for _ in 0..script_count {
            let name = name_context.scope(|| Name::read_options(reader, endian, ()))?;
            let has_name_string = name_context.resolve(&name).is_some();
            if !has_name_string && missing_file_names_seen.insert(name) {
                missing_file_names.push(name);
            }
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
            let decoded_data = decode_cps_script(
                &mut Cursor::new(uncompressed_data),
                endian,
                name_context,
                &mut missing_command_names,
                &mut missing_command_names_seen,
            )?;

            let name_string = name.with_context(name_context).to_string();
            let path = if has_name_string {
                PathBuf::from(name_string)
            } else {
                PathBuf::from(format!("{name_string}.tsc"))
            };
            cps.tscs.insert(path, decoded_data);
        }

        if !missing_command_names.is_empty() || !missing_file_names.is_empty() {
            eprintln!("CPS names without strings:");
            for missing_name in missing_command_names {
                eprintln!("  command name: {}", missing_name);
            }
            for missing_name in missing_file_names {
                eprintln!("  file name: {}", missing_name);
            }
        }

        Ok(cps)
    }
}

impl BinWrite for Cps {
    type Args<'a> = (&'a NameContext,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        (name_context,): Self::Args<'_>,
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
            let name = name_context.parse_or_hash_name(path.file_stem().unwrap().to_str().unwrap());
            let mut encoded_data = Cursor::new(Vec::new());
            encode_cps_script(script, &mut encoded_data, endian, name_context)?;
            let encoded_data = encoded_data.into_inner();
            let uncompressed_size = encoded_data.len() as u32;
            let mut compressed_data = Cursor::new(Vec::new());
            lzo_compress(&encoded_data, &mut compressed_data).unwrap();
            let compressed_data = compressed_data.into_inner();
            let compressed_size = compressed_data.len() as u32;

            name_context.scope(|| name.write_options(writer, endian, ()))?;
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
    pub fn read<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        name_context: &NameContext,
    ) -> BffResult<Self> {
        let first_char = u8::read_options(reader, endian, ())?;
        reader.seek(SeekFrom::Start(0))?;
        if first_char == CPS_FIRST_CHAR {
            Ok(<Self as BinRead>::read_options(
                reader,
                endian,
                (name_context,),
            )?)
        } else {
            let mut cps_data = Cursor::new(cps_crypt(reader, endian, ())?);
            Ok(<Self as BinRead>::read_options(
                &mut cps_data,
                endian,
                (name_context,),
            )?)
        }
    }

    pub fn write<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        unencrypted: bool,
        name_context: &NameContext,
    ) -> BffResult<()> {
        if unencrypted {
            <Self as BinWrite>::write_options(self, writer, endian, (name_context,))?;
        } else {
            let mut cps_data = Cursor::new(Vec::new());
            <Self as BinWrite>::write_options(self, &mut cps_data, endian, (name_context,))?;
            cps_data.seek(SeekFrom::Start(0))?;
            let encrypted_data = cps_crypt(&mut cps_data, endian, ())?;
            writer.write_all(&encrypted_data)?;
        }

        Ok(())
    }
}
