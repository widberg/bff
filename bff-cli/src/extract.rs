use bff::bigfile::*;
use binrw::io::BufReader;
use serde_json::to_string_pretty;
use std::fs::File;
use std::path::PathBuf;

pub fn extract(bigfile: &PathBuf, _directory: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(bigfile)?;
    let mut reader = BufReader::new(f);
    let bigfile_struct = BigFile::read(&mut reader)?;
    println!("{}", to_string_pretty(&bigfile_struct)?);
    Ok(())
}
