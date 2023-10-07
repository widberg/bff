use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use bff::bigfile::BigFile;

use crate::error::BffCliResult;
use crate::extract::read_bigfile;

pub fn write_bigfile(bigfile_path: &Path, bigfile: &BigFile) -> BffCliResult<()> {
    let mut writer = File::create(bigfile_path)?;
    // Intentionally use an unbuffered writer for debugging purposes.
    Ok(BigFile::write(bigfile, &mut writer)?)
}

pub fn round_trip(bigfile_path: &Path) -> BffCliResult<()> {
    let bigfile = read_bigfile(bigfile_path)?;
    let mut new_extension = bigfile_path
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_os_string();
    new_extension.push(".roundtrip");
    let output_bigfile_path = bigfile_path.with_extension(new_extension);
    write_bigfile(&output_bigfile_path, &bigfile)?;
    Ok(())
}
