use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use bff::bigfile::*;

pub fn extract(bigfile: &PathBuf, directory: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(bigfile)?;
    let mut reader = BufReader::new(f);
    let bigfile_struct = BigFile::read(&mut reader)?;
    print!("{:?}", bigfile_struct);
    Ok(())
}
