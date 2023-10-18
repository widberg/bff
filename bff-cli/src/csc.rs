use std::io;
use std::io::BufWriter;

use bff::tsc::csc_copy;
use bff::BufReader;

use crate::error::BffCliResult;

pub fn csc() -> BffCliResult<()> {
    let stdin = BufReader::new(io::stdin().lock());
    let mut stdout = BufWriter::new(io::stdout().lock());

    csc_copy(stdin, &mut stdout)?;

    Ok(())
}
