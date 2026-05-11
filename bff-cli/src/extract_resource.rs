use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, BffResource, BffResourceHeader};
use bff::bigfile::versions::Version;
use bff::class::Class;
use bff::names::{NameContext, NameType};
use bff::traits::{Artifact, Export, FromResource};

use crate::error::BffCliResult;
use crate::extract::read_in_names;

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

pub fn extract_resource(
    resource_path: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    let mut probe_reader = BufReader::new(File::open(resource_path)?);
    let name_type = BffResourceHeader::probe_name_type(&mut probe_reader)?;
    validate_version_override_name_type(version_override, name_type)?;
    let mut name_context = NameContext::new(name_type);
    read_in_names(in_names, &mut name_context)?;

    let f = File::open(resource_path)?;
    let mut reader = BufReader::new(f);
    let bff_resource = BffResource::read(&mut reader, &name_context)?;

    let platform = platform_override.unwrap_or(bff_resource.header.platform);
    let version = version_override
        .as_ref()
        .unwrap_or(&bff_resource.header.version);
    let header = BffResourceHeader {
        platform,
        version: version.clone(),
    };

    let class: Class = Class::from_resource(
        &bff_resource.resource,
        version.clone(),
        platform,
        &name_context,
    )?;
    let bff_class = BffClass { header, class };

    std::fs::create_dir(directory)?;

    let resource_serialized_path = directory.join("resource.json");
    let resource_serialized_writer = BufWriter::new(File::create(resource_serialized_path)?);
    bff::names::json::to_writer_pretty(resource_serialized_writer, &bff_class, &name_context)?;

    if let Ok(artifacts) = bff_class.class.export() {
        for (name, artifact) in artifacts {
            let artifact_path = directory.join(name);

            match artifact {
                Artifact::Binary(bytes) => {
                    std::fs::write(artifact_path.with_extension("bin"), bytes)?
                }
                Artifact::Dds(bytes) => std::fs::write(artifact_path.with_extension("dds"), bytes)?,
                Artifact::Wav(bytes) => std::fs::write(artifact_path.with_extension("wav"), bytes)?,
                Artifact::Text(text) => std::fs::write(artifact_path.with_extension("txt"), text)?,
            }
        }
    }

    Ok(())
}
