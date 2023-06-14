use bff::bigfile::*;
use binrw::io::BufReader;
use binrw::{BinRead, Endian};
use serde_json::to_string_pretty;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

pub fn info(bigfile: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(bigfile)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let elapsed = now.elapsed();
    let bigfile = BigFile::read_options(&mut reader, Endian::Little, ())?;
    println!("{}", to_string_pretty(&bigfile)?);
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
