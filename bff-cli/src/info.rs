use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use bff::bigfile::BigFile;
use bff::platforms::Platform;
use bff::BufReader;
use serde_json::to_writer_pretty;

pub fn info(
    bigfile_path: &Path,
    in_names: &Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for in_name in in_names {
        let f = File::open(in_name)?;
        let mut reader = BufReader::new(f);
        bff::names::names().lock().unwrap().read(&mut reader)?;
    }

    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    let bigfile = BigFile::read_platform(&mut reader, platform)?;
    to_writer_pretty(io::stdout().lock(), &bigfile)?;
    Ok(())
}
