use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, Resource};
use bff::bigfile::versions::Version;
use bff::traits::{Artifact, Import, TryIntoVersionPlatform};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::BffCliResult;
use crate::extract::write_names;

pub fn create(
    directory: &Path,
    bigfile_path: &Path,
    out_names: &Option<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading manifest");
    let manifest_path = directory.join("manifest.json");
    let manifest_reader = BufReader::new(File::open(manifest_path)?);
    let manifest = serde_json::from_reader(manifest_reader)?;

    let mut bigfile = BigFile {
        manifest,
        objects: Default::default(),
    };

    let resources_path = directory.join("resources");
    std::fs::create_dir_all(&resources_path)?;

    progress_bar.set_style(ProgressStyle::default_bar());
    progress_bar.set_length(std::fs::read_dir(&resources_path)?.count() as u64);

    for file in std::fs::read_dir(resources_path)? {
        let path = file?.path();
        progress_bar.inc(1);
        if path.is_file() {
            let mut file_reader = BufReader::new(File::open(&path)?);
            let resource = bigfile.read_bff_resource(&mut file_reader)?;
            if bigfile.objects.contains_key(&resource.name) {
                return Err(crate::error::BffCliError::DuplicateResource {
                    name: resource.name,
                });
            }
            bigfile.objects.insert(resource.name, resource);
        } else if path.is_dir() {
            let directory = path;
            let resource_serialized_path = directory.join("resource.json");
            let resource_serialized_reader = BufReader::new(File::open(resource_serialized_path)?);
            let mut bff_class: BffClass = serde_json::from_reader(resource_serialized_reader)?;

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
                (&bff_class.class).try_into_version_platform(version.clone(), platform)?;

            if bigfile.objects.contains_key(&resource.name) {
                return Err(crate::error::BffCliError::DuplicateResource {
                    name: resource.name,
                });
            }
            bigfile.objects.insert(resource.name, resource);
        }
    }

    progress_bar.set_style(ProgressStyle::default_spinner());
    progress_bar.set_message("Writing BigFile");

    let mut bigfile_writer = BufWriter::new(File::create(bigfile_path)?);
    bigfile.write(
        &mut bigfile_writer,
        *platform_override,
        version_override,
        None,
    )?;

    progress_bar.set_message("Writing names");

    if let Some(out_names) = out_names {
        write_names(out_names, &Some(bigfile.objects.keys().collect()))?;
    }

    progress_bar.finish_and_clear();

    Ok(())
}
