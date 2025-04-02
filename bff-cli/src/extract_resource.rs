use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, BffResource, BffResourceHeader};
use bff::bigfile::versions::Version;
use bff::class::Class;
use bff::traits::{Artifact, Export, TryIntoVersionPlatform};

use crate::error::BffCliResult;
use crate::extract::read_in_names;

pub fn extract_resource(
    resource_path: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    read_in_names(in_names)?;

    let f = File::open(resource_path)?;
    let mut reader = BufReader::new(f);
    let bff_resource = BffResource::read(&mut reader)?;

    let platform = platform_override.unwrap_or(bff_resource.header.platform);
    let version = version_override
        .as_ref()
        .unwrap_or(&bff_resource.header.version);
    let header = BffResourceHeader {
        platform,
        version: version.clone(),
    };

    let class: Class =
        (&bff_resource.resource).try_into_version_platform(version.clone(), platform)?;
    let bff_class = BffClass { header, class };

    std::fs::create_dir(directory)?;

    let resource_serialized_path = directory.join("resource.json");
    let resource_serialized_writer = BufWriter::new(File::create(resource_serialized_path)?);
    serde_json::to_writer_pretty(resource_serialized_writer, &bff_class)?;

    if let Ok(artifacts) = bff_class.class.export() {
        for (name, artifact) in artifacts {
            let artifact_path = directory.join(name);

            match artifact {
                Artifact::Binary(bytes) => {
                    std::fs::write(artifact_path.with_extension("bin"), bytes)?
                }
                Artifact::Text(text) => std::fs::write(artifact_path.with_extension("txt"), text)?,
            }
        }
    }

    Ok(())
}
