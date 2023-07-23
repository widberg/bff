use std::fs::File;
use std::path::Path;
use std::time::Instant;

use bff::bigfile::BigFile;
use bff::platforms::{extension_to_platform, Platform};
use bff::BufReader;
use serde_json::to_string_pretty;

pub fn extract(bigfile_path: &Path, _directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let platform = match bigfile_path.extension() {
        Some(extension) => extension_to_platform(extension).unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let bigfile = BigFile::read_platform(&mut reader, platform)?;
    let elapsed = now.elapsed();
    println!("{}", to_string_pretty(&bigfile)?);
    println!("Time to parse: {:?}", elapsed);
    Ok(())
}
