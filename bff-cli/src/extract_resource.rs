use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::resource::bff_resource::{BffResource, BffResourceHeader};
use bff::names::NameContext;
use bff::traits::Export as _;

use crate::error::BffCliResult;
use crate::shared::{read_in_names, resource_json_path, write_artifacts};

pub fn extract_resource(
    resource_path: &Path,
    directory: &Path,
    in_names: &[PathBuf],
) -> BffCliResult<()> {
    let mut probe_reader = BufReader::new(File::open(resource_path)?);
    let name_type = BffResourceHeader::probe_name_type(&mut probe_reader)?;
    let mut name_context = NameContext::new(name_type);
    read_in_names(in_names, &mut name_context)?;

    let f = File::open(resource_path)?;
    let mut reader = BufReader::new(f);
    let bff_resource = BffResource::read(&mut reader, &name_context)?;

    let bff_class = bff_resource.bff_class(&name_context)?;

    std::fs::create_dir(directory)?;

    let resource_serialized_path = resource_json_path(directory);
    let resource_serialized_writer = BufWriter::new(File::create(resource_serialized_path)?);
    bff::names::json::to_writer_pretty(resource_serialized_writer, &bff_class, &name_context)?;

    if let Ok(artifacts) = bff_class.class.export() {
        write_artifacts(directory, artifacts)?;
    }

    Ok(())
}
