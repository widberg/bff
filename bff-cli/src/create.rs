use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, Resource};
use bff::bigfile::versions::Version;
use bff::names::{NameContext, NameType};
use bff::traits::{Artifact, Import, ToResource};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::BffCliResult;
use crate::extract::write_names;

fn validate_version_override_name_type(
    version_override: &Option<Version>,
    expected_name_type: NameType,
) -> BffCliResult<()> {
    if let Some(version_override) = version_override {
        let override_name_type = version_override.name_type()?;
        if override_name_type != expected_name_type {
            return Err(std::io::Error::other(format!(
                "`--version-override` implies NameType {:?}, but context requires {:?}",
                override_name_type, expected_name_type
            ))
            .into());
        }
    }

    Ok(())
}

pub fn create(
    directory: &Path,
    bigfile_path: &Path,
    out_names: &Option<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
    version_to_write: &Option<Version>,
    tag: &Option<String>,
) -> BffCliResult<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading manifest");
    let manifest_path = directory.join("manifest.json");
    let manifest_name_type = bff::names::json::probe_name_type_from_manifest_reader(
        BufReader::new(File::open(&manifest_path)?),
    )?;
    validate_version_override_name_type(version_override, manifest_name_type)?;
    let mut name_context = NameContext::new(manifest_name_type);
    let manifest_reader = BufReader::new(File::open(manifest_path)?);
    let manifest = bff::names::json::from_reader(manifest_reader, &mut name_context)?;

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
            let resource = Resource::read_bff_resource(&mut file_reader, &name_context)?;
            if resources.contains_key(&resource.name) {
                return Err(crate::error::BffCliError::DuplicateResource {
                    name: resource.name,
                });
            }
            resources.insert(resource.name, resource);
        } else if path.is_dir() {
            let directory = path;
            let resource_serialized_path = directory.join("resource.json");
            let resource_serialized_reader = BufReader::new(File::open(resource_serialized_path)?);
            let mut bff_class: BffClass =
                bff::names::json::from_reader(resource_serialized_reader, &mut name_context)?;

            let mut artifacts = HashMap::new();

            for file in std::fs::read_dir(directory)? {
                let path = file?.path();
                if path.is_file() {
                    if path.file_name() == Some("resource.json".as_ref()) {
                        continue;
                    }
                    match path.extension().unwrap().to_str().unwrap() {
                        "bin" => {
                            let name = path.file_stem().unwrap().to_os_string();
                            let bytes = std::fs::read(path)?;
                            artifacts.insert(name, Artifact::Binary(bytes));
                        }
                        "dds" => {
                            let name = path.file_stem().unwrap().to_os_string();
                            let bytes = std::fs::read(path)?;
                            artifacts.insert(name, Artifact::Dds(bytes));
                        }
                        "wav" => {
                            let name = path.file_stem().unwrap().to_os_string();
                            let bytes = std::fs::read(path)?;
                            artifacts.insert(name, Artifact::Wav(bytes));
                        }
                        "txt" => {
                            let name = path.file_stem().unwrap().to_os_string();
                            let text = std::fs::read_to_string(path)?;
                            artifacts.insert(name, Artifact::Text(text));
                        }
                        _ => {}
                    }
                }
            }

            let _ = bff_class.class.import(&artifacts);

            let platform = platform_override.unwrap_or(bff_class.header.platform);
            let version = version_override
                .as_ref()
                .unwrap_or(&bff_class.header.version);

            let resource: Resource =
                bff_class
                    .class
                    .to_resource(version, platform, &name_context)?;

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
    bigfile.write(
        &mut bigfile_writer,
        *platform_override,
        version_override,
        version_to_write,
        tag.as_deref(),
        &name_context,
    )?;

    progress_bar.set_message("Writing names");

    if let Some(out_names) = out_names {
        let resource_names: Vec<_> = bigfile.resource_names().collect();
        write_names(out_names, Some(resource_names.as_slice()), &name_context)?;
    }

    progress_bar.finish_and_clear();

    Ok(())
}
