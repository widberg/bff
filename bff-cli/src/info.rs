use std::fs::File;
use std::io;
use std::path::Path;
use std::time::Instant;

use bff::bigfile::BigFile;
use bff::platforms::extension_to_endian;
use bff::{BufReader, Endian};
use serde_json::to_writer_pretty;

pub fn info(bigfile_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let endian = match bigfile_path.extension() {
        Some(extension) => extension_to_endian(extension).unwrap_or(Endian::Little),
        None => Endian::Little,
    };
    let f = File::open(bigfile_path)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let elapsed = now.elapsed();
    let bigfile = BigFile::read_endian(&mut reader, endian)?;
    to_writer_pretty(io::stdout().lock(), &bigfile)?;
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
