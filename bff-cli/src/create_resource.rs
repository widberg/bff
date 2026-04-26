use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, Resource};
use bff::bigfile::versions::Version;
use bff::names::{NameContext, NameType};
use bff::traits::{Artifact, Import, IntoResource};

use crate::error::BffCliResult;
use crate::extract::write_names;

fn validate_version_override_name_type(
    version_override: &Option<Version>,
    expected_name_type: NameType,
) -> BffCliResult<()> {
    if let Some(version_override) = version_override {
        let override_name_type: NameType = version_override.try_into()?;
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

pub fn create_resource(
    directory: &Path,
    resource_path: &Path,
    out_names: &Option<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    let resource_serialized_path = directory.join("resource.json");
    let name_type = bff::names::json::probe_name_type_from_bff_class_reader(BufReader::new(
        File::open(&resource_serialized_path)?,
    ))?;
    validate_version_override_name_type(version_override, name_type)?;
    let mut name_context = NameContext::new(name_type);
    let resource_serialized_reader = BufReader::new(File::open(&resource_serialized_path)?);
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
            .into_resource(version.clone(), platform, &name_context)?;

    let mut resource_writer = BufWriter::new(File::create(resource_path)?);
    Resource::dump_bff_resource(
        &resource,
        &mut resource_writer,
        platform,
        version,
        &name_context,
    )?;

    if let Some(out_names) = out_names {
        write_names(out_names, None, &name_context)?;
    }

    Ok(())
}
