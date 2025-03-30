use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bff::tsc::Cps;
use bff::{BufReader, Endian};

use crate::error::BffCliResult;
use crate::lz::LzEndian;

pub fn extract_cps(cps: &Path, directory: &Path, endian: &LzEndian) -> BffCliResult<()> {
    let endian: Endian = (*endian).into();
    let mut cps_reader = BufReader::new(File::open(cps)?);
    let cps = Cps::read(&mut cps_reader, endian)?;

    for (path, data) in cps.tscs {
        let tsc_path = directory.join(path);
        let prefix = tsc_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let tsc = File::create(tsc_path)?;
        let mut tsc_writer = BufWriter::new(tsc);
        tsc_writer.write_all(data.as_bytes())?;
    }

    Ok(())
}

fn read_files_into_cps_recursively(
    cps: &mut Cps,
    directory: &Path,
    base: &Path,
) -> BffCliResult<()> {
    let paths = std::fs::read_dir(directory)?;
    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            read_files_into_cps_recursively(cps, &path, base)?;
        } else {
            let tsc = std::fs::read_to_string(&path)?;
            let relative_path = path.strip_prefix(base)?.to_path_buf();
            cps.tscs.insert(relative_path, tsc);
        }
    }
    Ok(())
}

pub fn create_cps(
    directory: &Path,
    cps_path: &Path,
    endian: &LzEndian,
    unencrypted: &bool,
) -> BffCliResult<()> {
    let endian: Endian = (*endian).into();
    let mut cps = Cps::default();
    read_files_into_cps_recursively(&mut cps, directory, directory)?;

    let mut cps_writer = BufWriter::new(File::create(cps_path)?);

    cps.write(&mut cps_writer, endian, *unencrypted)?;

    Ok(())
}
