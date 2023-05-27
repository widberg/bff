use bff::bigfile::*;
use binrw::io::BufReader;
use serde_json::to_string_pretty;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

pub fn extract(bigfile: &PathBuf, _directory: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(bigfile)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let elapsed = now.elapsed();
    let bigfile_struct = BigFile::read(&mut reader)?;
    println!("{}", to_string_pretty(&bigfile_struct)?);
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
