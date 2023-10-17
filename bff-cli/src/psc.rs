use std::fs::File;
use std::path::Path;
use bff::BufReader;
use crate::error::BffCliResult;

pub fn extract_psc(
    psc: &Path,
    _directory: &Path,
) -> BffCliResult<()> {
    let mut psc_reader = BufReader::new(File::open(psc)?);
    let psc = bff::psc::Psc::read(&mut psc_reader)?;

    println!("{:#?}", psc);

    Ok(())
}
