use std::fs::File;
use std::io;
use std::path::Path;
use std::time::Instant;

use bff::bigfile::BigFile;
use bff::platforms::Platform;
use bff::BufReader;
use serde_json::to_writer_pretty;

pub fn info(bigfile_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let bigfile = BigFile::read_platform(&mut reader, platform)?;
    let elapsed = now.elapsed();
    to_writer_pretty(io::stdout().lock(), &bigfile)?;
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
