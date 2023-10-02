use std::io;
use std::path::{Path, PathBuf};

use serde_json::to_writer_pretty;

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_names};

pub fn info(bigfile_path: &Path, in_names: &Vec<PathBuf>) -> BffCliResult<()> {
    read_names(bigfile_path, in_names)?;

    let bigfile = read_bigfile(bigfile_path)?;
    to_writer_pretty(io::stdout().lock(), &bigfile)?;
    Ok(())
}
