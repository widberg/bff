use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use binrw::{BinRead, BinResult, BinWrite, Endian, NullString, args};

use crate::BffResult;
use crate::helpers::copy_repeat;
use crate::lz::{lzo_compress, lzo_decompress};
use crate::names::{Name, NameContext};

const DEFAULT_CPS_IN_NAMES: &str = include_str!("ALLSCRIPTS.CPSNameWii");

pub fn read_default_cps_names(name_context: &mut NameContext) -> BffResult<()> {
    let mut reader = crate::BufReader::new(Cursor::new(DEFAULT_CPS_IN_NAMES.as_bytes()));
    name_context.read(&mut reader)?;
    Ok(())
}

#[derive(Debug, Default, Eq, PartialEq)]
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

fn parse_opal_float_token(token: &str) -> Option<f32> {
    token.replace('f', "").parse::<f32>().ok()
}

fn is_opal_bool_token(token: &str) -> bool {
    let upper = token.to_ascii_uppercase();
    upper == "TRUE" || upper == "FALSE" || upper == "ON" || upper == "OFF"
}

fn format_cps_string_param(value: &str) -> String {
    let needs_quotes = value.is_empty()
        || value.chars().any(|c| c.is_whitespace())
        || value.contains('"')
        || parse_opal_float_token(value).is_some()
        || is_opal_bool_token(value);

    if needs_quotes {
        format!("\"{value}\"")
    } else {
        value.to_owned()
    }
}

fn remove_cpp_style_comments(script: &str) -> String {
    let mut result = String::new();
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        result.push_str(trimmed);
        result.push('\n');
    }
    result
}

fn remove_c_style_comments(script: &str) -> String {
    let mut offset = 0usize;
    let mut output = String::new();

    loop {
        let next_start = script[offset..].find("/*").map(|i| offset + i);
        let next_end = script[offset..].find("*/").map(|i| offset + i);

        match (next_start, next_end) {
            (None, None) => {
                output.push_str(&script[offset..]);
                break;
            }
            (Some(start), None) => {
                output.push_str(&script[offset..start]);
                break;
            }
            (None, Some(end)) => {
                let end_inclusive = end + 2;
                output.push_str(&script[offset..end_inclusive]);
                offset = end_inclusive;
            }
            (Some(start), Some(end)) if start < end => {
                output.push_str(&script[offset..start]);
                offset = end + 2;
            }
            (Some(_), Some(end)) => {
                let end_inclusive = end + 2;
                output.push_str(&script[offset..end_inclusive]);
                offset = end_inclusive;
            }
        }

        if offset >= script.len() {
            break;
        }
    }

    output
}

fn split_arguments_opal(line: &str) -> Vec<String> {
    let mut chars = line.chars().collect::<Vec<_>>();
    let mut in_quoted_string = false;
    let separator = '\u{00FF}';
    for c in &mut chars {
        if *c == '"' {
            in_quoted_string = !in_quoted_string;
        }
        if (*c == ' ' || *c == '\t') && !in_quoted_string {
            *c = separator;
        }
    }

    chars
        .into_iter()
        .collect::<String>()
        .split(separator)
        .filter(|part| !part.is_empty())
        .map(|part| part.replace('"', ""))
        .collect()
}

fn decode_cps_script<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    name_context: &NameContext,
) -> BinResult<String> {
    let mut script = String::new();

    let num_lines = u32::read_options(reader, endian, ())?;

    for _ in 0..num_lines {
        let num_params = u8::read_options(reader, endian, ())?;
        if num_params != 0 {
            let command_name = name_context.scope(|| Name::read_options(reader, endian, ()))?;
            script.push_str(command_name.with_context(name_context).to_string().as_str());
            for _ in 1..num_params {
                script.push(' ');
                let param = Param::read_options(reader, endian, ())?;
                match param {
                    Param::String(s) => {
                        script.push_str(format_cps_string_param(&s.to_string()).as_str())
                    }
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
    name_context: &mut NameContext,
) -> BinResult<()> {
    let start = writer.stream_position()?;
    let mut num_lines: u32 = 0;
    num_lines.write_options(writer, endian, ())?;

    let script = remove_c_style_comments(&remove_cpp_style_comments(script));

    for line in script.lines() {
        if line.is_empty() {
            continue;
        }

        let args = split_arguments_opal(line);
        if args.is_empty() {
            continue;
        }
        if args.len() > 32 {
            eprintln!("ERROR: Command line has more than 32 parameters; skipping line: {line}");
            continue;
        }

        let command_name_token = args[0].to_ascii_uppercase();
        if command_name_token.contains('%') {
            eprintln!("ERROR: Command names should not use '%': {command_name_token}");
        }
        let command_name = name_context.parse_i32_or_hash_name(&command_name_token);
        let mut params: Vec<Param> = Vec::with_capacity(args.len().saturating_sub(1));
        for param_token in args.into_iter().skip(1) {
            let upper = param_token.to_ascii_uppercase();
            if let Some(param) = parse_opal_float_token(&param_token) {
                params.push(Param::Float(param));
            } else if upper == "TRUE" || upper == "ON" {
                params.push(Param::Float(1.0));
            } else if upper == "FALSE" || upper == "OFF" {
                params.push(Param::Float(0.0));
            } else {
                params.push(Param::String(param_token.into()));
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

        let _version = <[u8; 8]>::read_options(reader, endian, ())?;
        let _flags = u32::read_options(reader, endian, ())?;
        let script_count = u32::read_options(reader, endian, ())?;

        for _ in 0..script_count {
            let name = name_context.scope(|| Name::read_options(reader, endian, ()))?;
            let has_name_string = name_context.resolve(&name).is_some();
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
            let decoded_data =
                decode_cps_script(&mut Cursor::new(uncompressed_data), endian, name_context)?;

            let name_string = name.with_context(name_context).to_string();
            let path = if has_name_string {
                PathBuf::from(name_string)
            } else {
                PathBuf::from(format!("{name_string}.tsc"))
            };
            cps.tscs.insert(path, decoded_data);
        }

        Ok(cps)
    }
}

impl BinWrite for Cps {
    type Args<'a> = (&'a mut NameContext,);

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

        let mut scripts = self
            .tscs
            .iter()
            .map(|(path, script)| {
                let path_string = path.to_string_lossy();
                let is_bare_numeric_tsc = path.components().count() == 1
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("tsc"))
                        .unwrap_or(false)
                    && path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|stem| stem.parse::<i32>().ok())
                        .is_some();

                let name = if is_bare_numeric_tsc {
                    let stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap();
                    name_context.parse_i32_or_hash_name(stem)
                } else {
                    name_context.parse_i32_or_hash_name(path_string.as_ref())
                };
                // Sort by unsigned hash value for deterministic CPS ordering.
                let sort_key = name.get_value() as u64;
                (sort_key, name, path, script)
            })
            .collect::<Vec<_>>();
        scripts.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.2.cmp(b.2)));

        for (_, name, _, script) in scripts {
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
pub fn cps_copy<R: Read, W: Write>(reader: R, writer: &mut W) -> BffResult<()> {
    let mut seed = CPS_FIRST_CHAR;
    for byte in reader.bytes() {
        let byte = byte?;
        writer.write_all(&[byte ^ seed])?;
        seed = seed.wrapping_add(CPS_SEED_STEP);
    }

    Ok(())
}

pub fn cps_buffer(data: &mut [u8]) {
    let mut seed = CPS_FIRST_CHAR;
    for byte in data {
        *byte ^= seed;
        seed = seed.wrapping_add(CPS_SEED_STEP);
    }
}

#[allow(clippy::unbuffered_bytes)]
#[binrw::parser(reader)]
fn cps_crypt() -> BinResult<Vec<u8>> {
    let mut data = Vec::new();

    reader.read_to_end(&mut data)?;
    cps_buffer(&mut data);

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
        name_context: &mut NameContext,
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
