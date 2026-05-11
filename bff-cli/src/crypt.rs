use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

use bff::BufReader;
use bff::tsc::{
    cps_copy,
    csc_copy,
    mqfel_settings_bin_decrypt_buffer,
    mqfel_settings_bin_encrypt_buffer,
};
use clap::ValueEnum;

use crate::error::BffCliResult;
use crate::stdio_or_path::StdioOrPath;

#[derive(ValueEnum, Clone, Copy)]
pub enum CryptAlgorithm {
    Csc,
    Cps,
    #[value(alias = "settings.bin")]
    MqfelSettingsBin,
}

#[derive(Clone, Copy)]
enum CryptDirection {
    Crypt,
    Uncrypt,
}

fn crypt_internal<R: Read, W: Write>(
    input: &mut R,
    output: &mut W,
    algorithm: CryptAlgorithm,
    key: u8,
    direction: CryptDirection,
) -> BffCliResult<()> {
    match algorithm {
        CryptAlgorithm::Csc => csc_copy(input, output, key)?,
        CryptAlgorithm::Cps => cps_copy(input, output)?,
        CryptAlgorithm::MqfelSettingsBin => {
            let mut data = Vec::new();
            input.read_to_end(&mut data)?;
            match direction {
                CryptDirection::Crypt => mqfel_settings_bin_encrypt_buffer(&mut data),
                CryptDirection::Uncrypt => mqfel_settings_bin_decrypt_buffer(&mut data),
            }
            output.write_all(&data)?;
        }
    }

    Ok(())
}

fn process(
    input: StdioOrPath,
    output: StdioOrPath,
    algorithm: CryptAlgorithm,
    key: u8,
    direction: CryptDirection,
) -> BffCliResult<()> {
    match (input, output) {
        (StdioOrPath::Stdio, StdioOrPath::Stdio) => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            crypt_internal(
                &mut stdin.lock(),
                &mut stdout.lock(),
                algorithm,
                key,
                direction,
            )
        }
        (StdioOrPath::Stdio, StdioOrPath::Path(output_path)) => {
            let stdin = io::stdin();
            let mut output = BufWriter::new(File::create(output_path)?);
            crypt_internal(&mut stdin.lock(), &mut output, algorithm, key, direction)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Stdio) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let stdout = io::stdout();
            crypt_internal(&mut input, &mut stdout.lock(), algorithm, key, direction)
        }
        (StdioOrPath::Path(input_path), StdioOrPath::Path(output_path)) => {
            let mut input = BufReader::new(File::open(input_path)?);
            let mut output = BufWriter::new(File::create(output_path)?);
            crypt_internal(&mut input, &mut output, algorithm, key, direction)
        }
    }
}

pub fn crypt(
    uncrypted: StdioOrPath,
    crypted: StdioOrPath,
    algorithm: CryptAlgorithm,
    key: u8,
) -> BffCliResult<()> {
    process(uncrypted, crypted, algorithm, key, CryptDirection::Crypt)
}

pub fn uncrypt(
    crypted: StdioOrPath,
    uncrypted: StdioOrPath,
    algorithm: CryptAlgorithm,
    key: u8,
) -> BffCliResult<()> {
    process(crypted, uncrypted, algorithm, key, CryptDirection::Uncrypt)
}
