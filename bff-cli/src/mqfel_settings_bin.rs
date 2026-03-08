use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use bff::BufReader;
use bff::tsc::{mqfel_settings_bin_create_from_directory, mqfel_settings_bin_extract_to_directory};

use crate::error::BffCliResult;

pub fn extract_mqfel_settings_bin(settings_bin: &Path, directory: &Path) -> BffCliResult<()> {
    let reader = BufReader::new(File::open(settings_bin)?);
    mqfel_settings_bin_extract_to_directory(reader, directory)?;
    Ok(())
}

pub fn create_mqfel_settings_bin(directory: &Path, settings_bin: &Path) -> BffCliResult<()> {
    let mut writer = BufWriter::new(File::create(settings_bin)?);
    mqfel_settings_bin_create_from_directory(directory, &mut writer)?;
    Ok(())
}
