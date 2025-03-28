use std::fs::File;
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

use serde_json::to_writer_pretty;

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_names};

pub fn info(
    bigfile_path: &Path,
    in_names: &Vec<PathBuf>,
    out_reference_map: &Option<PathBuf>,
) -> BffCliResult<()> {
    read_names(bigfile_path, in_names)?;

    let bigfile = read_bigfile(bigfile_path)?;
    to_writer_pretty(io::stdout().lock(), &bigfile)?;

    if let Some(out_dependencies) = out_reference_map {
        let f = File::create(out_dependencies)?;
        let writer = BufWriter::new(f);
        to_writer_pretty(writer, &bigfile.reference_map())?;
    }

    Ok(())
}
