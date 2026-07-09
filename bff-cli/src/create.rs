use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use bff::bigfile::BigFile;
use bff::bigfile::manifest::Manifest;
use bff::bigfile::resource::bff_resource::BffResource;
use bff::bigfile::versions::Version;
use bff::class::bff_class::BffClass;
use bff::names::NameContext;
use bff::traits::Import as _;
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::{BffCliError, BffCliResult};
use crate::shared::{read_artifacts, resolve_platform, resource_json_path, write_names};

pub fn create(
    directory: &Path,
    bigfile_path: &Path,
    out_names: Option<&Path>,
    version_to_write: Option<&Version>,
    tag: Option<&str>,
) -> BffCliResult<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading manifest");
    let manifest_path = directory.join("manifest.json");
    let manifest_name_type = bff::names::json::probe_name_type_from_manifest_reader(
        BufReader::new(File::open(&manifest_path)?),
    )?;
    let mut name_context = NameContext::new(manifest_name_type);
    let manifest_reader = BufReader::new(File::open(manifest_path)?);
    let manifest: Manifest = bff::names::json::from_reader(manifest_reader, &mut name_context)?;

    let path_platform = resolve_platform(bigfile_path)?;
    if manifest.platform != path_platform {
        return Err(BffCliError::PlatformMismatch {
            manifest_platform: manifest.platform,
            path_platform,
            path: bigfile_path.to_path_buf(),
        });
    }

    let resources_path = directory.join("resources");
    std::fs::create_dir_all(&resources_path)?;

    progress_bar.set_style(ProgressStyle::default_bar());
    progress_bar.set_length(std::fs::read_dir(&resources_path)?.count() as u64);

    let mut resources = HashMap::new();
    for file in std::fs::read_dir(resources_path)? {
        let path = file?.path();
        progress_bar.inc(1);
        if path.is_file() {
            let mut file_reader = BufReader::new(File::open(&path)?);
            let BffResource { resource, .. } = BffResource::read(&mut file_reader, &name_context)?;
            if resources.contains_key(&resource.name) {
                return Err(crate::error::BffCliError::DuplicateResource {
                    name: resource.name,
                });
            }
            resources.insert(resource.name, resource);
        } else if path.is_dir() {
            let directory = path;
            let resource_serialized_path = resource_json_path(&directory);
            let resource_serialized_reader = BufReader::new(File::open(resource_serialized_path)?);
            let mut bff_class: BffClass =
                bff::names::json::from_reader(resource_serialized_reader, &mut name_context)?;

            let artifacts = read_artifacts(&directory)?;

            let _ = bff_class.class.import(&artifacts);

            let BffResource { resource, .. } = bff_class.bff_resource(&name_context)?;

            if resources.contains_key(&resource.name) {
                return Err(crate::error::BffCliError::DuplicateResource {
                    name: resource.name,
                });
            }
            resources.insert(resource.name, resource);
        }
    }
    let bigfile = BigFile::new(manifest, resources);

    progress_bar.set_style(ProgressStyle::default_spinner());
    progress_bar.set_message("Writing BigFile");

    let mut bigfile_writer = BufWriter::new(File::create(bigfile_path)?);
    bigfile.write(&mut bigfile_writer, version_to_write, tag, &name_context)?;

    progress_bar.set_message("Writing names");

    if let Some(out_names) = out_names {
        let resource_names: Vec<_> = bigfile.resource_names().collect();
        write_names(out_names, Some(resource_names.as_slice()), &name_context)?;
    }

    progress_bar.finish_and_clear();

    Ok(())
}
