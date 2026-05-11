use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use bff::petgraph::dot::{Config, Dot};

use crate::error::BffCliResult;
use crate::extract::{probe_bigfile_name_context, read_bigfile, read_bigfile_names, read_in_names};

pub fn info(
    bigfile_path: &Path,
    in_names: &[PathBuf],
    out_reference_graph: Option<&Path>,
) -> BffCliResult<()> {
    let mut name_context = probe_bigfile_name_context(bigfile_path, None, None)?;
    read_bigfile_names(bigfile_path, &mut name_context)?;
    read_in_names(in_names, &mut name_context)?;

    let bigfile = read_bigfile(bigfile_path, None, None, &name_context)?;
    bff::names::json::to_writer_pretty(io::stdout().lock(), bigfile.manifest(), &name_context)?;

    if let Some(out_dependencies) = out_reference_graph {
        let f = File::create(out_dependencies)?;
        let mut writer = BufWriter::new(f);
        let graph = bigfile.reference_graph(&name_context);
        let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
        name_context.scope(|| write!(&mut writer, "{:?}", dot))?;
    }

    Ok(())
}
