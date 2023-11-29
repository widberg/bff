use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bff::tsc::Psc;
use bff::BufReader;
use clap::ValueEnum;

use crate::error::BffCliResult;

#[derive(ValueEnum, Clone, Copy)]
pub enum PscAlgorithm {
    None,
    Lz4,
    Gzip,
}

impl From<PscAlgorithm> for bff::tsc::PscAlgorithm {
    fn from(algorithm: PscAlgorithm) -> Self {
        match algorithm {
            PscAlgorithm::None => Self::None,
            PscAlgorithm::Lz4 => Self::Lz4,
            PscAlgorithm::Gzip => Self::Gzip,
        }
    }
}

pub fn extract_psc(psc: &Path, directory: &Path, algorithm: &PscAlgorithm) -> BffCliResult<()> {
    let mut psc_reader = BufReader::new(File::open(psc)?);
    let psc = Psc::read(&mut psc_reader, (*algorithm).into())?;

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

fn read_files_into_psc_recursively(
    psc: &mut Psc,
    directory: &Path,
    base: &Path,
) -> BffCliResult<()> {
    let paths = std::fs::read_dir(directory)?;
    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            read_files_into_psc_recursively(psc, &path, base)?;
        } else {
            let tsc = std::fs::read_to_string(&path)?;
            let relative_path = path.strip_prefix(base)?.to_path_buf();
            psc.tscs.insert(relative_path, tsc);
        }
    }
    Ok(())
}

pub fn create_psc(directory: &Path, psc_path: &Path, algorithm: &PscAlgorithm) -> BffCliResult<()> {
    let mut psc = Psc::default();
    read_files_into_psc_recursively(&mut psc, directory, directory)?;

    let mut psc_writer = BufWriter::new(File::create(psc_path)?);

    psc.write(&mut psc_writer, (*algorithm).into())?;

    Ok(())
}
