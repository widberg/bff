use bff::bigfile::*;
use binrw::io::BufReader;
use binrw::{BinRead, Endian};
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

pub fn info(bigfile_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let endian = match bigfile_path.extension() {
        Some(extension) => extension_to_endian(extension).unwrap_or(Endian::Little),
        None => Endian::Little,
    };
    let f = File::open(bigfile_path)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let elapsed = now.elapsed();
    let bigfile = BigFile::read_options(&mut reader, endian, ())?;
    to_writer_pretty(io::stdout().lock(), &bigfile)?;
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
