use std::fs::File;
use std::io;
use std::io::BufWriter;

use bff::tsc::csc_copy;
use bff::BufReader;

use crate::error::BffCliResult;
use crate::stdio_or_path::StdioOrPath;

pub fn csc(input: &StdioOrPath, output: &StdioOrPath, key: &u8) -> BffCliResult<()> {
    match (input, output) {
        (StdioOrPath::Stdio, StdioOrPath::Stdio) => {
            let stdin = BufReader::new(io::stdin().lock());
            let mut stdout = BufWriter::new(io::stdout().lock());

            csc_copy(stdin, &mut stdout, *key)?;
        }
        (StdioOrPath::Stdio, StdioOrPath::Path(path)) => {
            let stdin = BufReader::new(io::stdin().lock());
            let mut stdout = BufWriter::new(File::create(path)?);

            csc_copy(stdin, &mut stdout, *key)?;
        }
        (StdioOrPath::Path(path), StdioOrPath::Stdio) => {
            let stdin = BufReader::new(File::open(path)?);
            let mut stdout = BufWriter::new(io::stdout().lock());

            csc_copy(stdin, &mut stdout, *key)?;
        }
        (StdioOrPath::Path(path), StdioOrPath::Path(path2)) => {
            let stdin = BufReader::new(File::open(path)?);
            let mut stdout = BufWriter::new(File::create(path2)?);

            csc_copy(stdin, &mut stdout, *key)?;
        }
    }

    Ok(())
}
