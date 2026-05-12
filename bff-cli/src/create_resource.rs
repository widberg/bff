use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use bff::BufReader;
use bff::bigfile::platforms::Platform;
use bff::bigfile::versions::Version;
use bff::class::bff_class::BffClass;
use bff::names::NameContext;
use bff::traits::Import as _;

use crate::error::BffCliResult;
use crate::shared::{
    read_artifacts,
    resource_json_path,
    validate_version_override_name_type,
    write_names,
};

pub fn create_resource(
    directory: &Path,
    resource_path: &Path,
    out_names: Option<&Path>,
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
) -> BffCliResult<()> {
    let resource_serialized_path = resource_json_path(directory);
    let name_type = bff::names::json::probe_name_type_from_bff_class_reader(BufReader::new(
        File::open(&resource_serialized_path)?,
    ))?;
    validate_version_override_name_type(version_override, name_type)?;
    let mut name_context = NameContext::new(name_type);
    let resource_serialized_reader = BufReader::new(File::open(&resource_serialized_path)?);
    let mut bff_class: BffClass =
        bff::names::json::from_reader(resource_serialized_reader, &mut name_context)?;

    let artifacts = read_artifacts(directory)?;

    let _ = bff_class.class.import(&artifacts);

    let bff_resource =
        bff_class.bff_resource_with_override(platform_override, version_override, &name_context)?;

    let mut resource_writer = BufWriter::new(File::create(resource_path)?);
    bff_resource.write(&mut resource_writer, &name_context)?;

    if let Some(out_names) = out_names {
        write_names(out_names, None, &name_context)?;
    }

    Ok(())
}
