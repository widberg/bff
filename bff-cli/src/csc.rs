use std::io;
use std::io::{BufWriter, Read, Write};

use bff::BufReader;

use crate::error::BffCliResult;

pub fn csc() -> BffCliResult<()> {
    let stdin = BufReader::new(io::stdin().lock());
    let mut stdout = BufWriter::new(io::stdout().lock());

    for byte in stdin.bytes() {
        let byte = byte?;
        stdout.write_all(&[!byte])?;
    }

    Ok(())
}
