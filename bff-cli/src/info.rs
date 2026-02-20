use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use bff::names::NameContext;
use bff::petgraph::dot::{Config, Dot};

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_bigfile_names, read_in_names};

pub fn info(
    bigfile_path: &Path,
    in_names: &Vec<PathBuf>,
    out_reference_graph: &Option<PathBuf>,
    name_context: &NameContext,
) -> BffCliResult<()> {
    read_bigfile_names(bigfile_path, name_context)?;
    read_in_names(in_names, name_context)?;

    let bigfile = read_bigfile(bigfile_path, &None, &None, name_context)?;
    bff::names::json::to_writer_pretty(io::stdout().lock(), &bigfile, name_context)?;

    if let Some(out_dependencies) = out_reference_graph {
        let f = File::create(out_dependencies)?;
        let mut writer = BufWriter::new(f);
        let graph = bigfile.reference_graph();
        let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
        write!(&mut writer, "{:?}", dot)?;
    }

    Ok(())
}
