use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, Resource};
use bff::bigfile::versions::Version;
use bff::traits::TryIntoVersionPlatform;

use crate::error::BffCliResult;
use crate::extract::write_names;

pub fn create_resource(
    directory: &Path,
    resource_path: &Path,
    out_names: &Option<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    let resource_serialized_path = directory.join("resource.json");
    let resource_serialized_reader = BufReader::new(File::open(&resource_serialized_path)?);
    let bff_class: BffClass = serde_json::from_reader(resource_serialized_reader)?;

    let platform = platform_override.unwrap_or(bff_class.header.platform);
    let version = version_override
        .as_ref()
        .unwrap_or(&bff_class.header.version);

    let resource: Resource =
        (&bff_class.class).try_into_version_platform(version.clone(), platform)?;

    let mut resource_writer = BufWriter::new(File::create(resource_path)?);
    Resource::dump_bff_resource(&resource, &mut resource_writer, platform, version)?;

    if let Some(out_names) = out_names {
        write_names(out_names, &None)?;
    }

    Ok(())
}
