use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use bff::bigfile::resource::BffClass;
use schemars::schema_for;

use crate::error::BffCliResult;

pub fn dump_json_schema(path: &Path) -> BffCliResult<()> {
    let json_schema_writer = BufWriter::new(File::create(path)?);

    let schema = schema_for!(BffClass);

    serde_json::to_writer_pretty(json_schema_writer, &schema)?;

    Ok(())
}
