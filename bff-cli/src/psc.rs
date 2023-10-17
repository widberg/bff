use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use bff::BufReader;
use crate::error::BffCliResult;

pub fn extract_psc(
    psc: &Path,
    directory: &Path,
) -> BffCliResult<()> {
    let mut psc_reader = BufReader::new(File::open(psc)?);
    let psc = bff::psc::Psc::read(&mut psc_reader)?;

    for (path, data) in psc.tscs {
        let tsc_path = directory.join(path);
        let prefix = tsc_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let tsc = File::create(tsc_path)?;
        let mut tsc_writer = BufWriter::new(tsc);
        tsc_writer.write_all(data.as_bytes())?;
    }

    Ok(())
}

pub fn create_psc(
    _directory: &Path,
    _psc: &Path,
) -> BffCliResult<()> {
    Ok(())
}
