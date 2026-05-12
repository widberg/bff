use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::BigFile;
use bff::bigfile::platforms::{Platform, try_platform_style_to_name_extension};
use bff::bigfile::versions::Version;
use bff::names::{Name, NameContext, NameType};
use bff::traits::Artifact;

use crate::error::BffCliResult;

pub const RESOURCE_JSON_FILE_NAME: &str = "resource.json";

pub fn validate_version_override_name_type(
    version_override: Option<&Version>,
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

pub fn resource_json_path(directory: &Path) -> PathBuf {
    directory.join(RESOURCE_JSON_FILE_NAME)
}

pub fn read_bigfile_names(bigfile_path: &Path, name_context: &mut NameContext) -> BffCliResult<()> {
    if let Some(extension) = bigfile_path.extension() {
        let name_extension =
            try_platform_style_to_name_extension(extension.try_into()?, extension.try_into()?)?;
        let in_name = bigfile_path.with_extension(name_extension);

        if let Ok(f) = File::open(in_name) {
            let mut reader = BufReader::new(f);
            name_context.read(&mut reader)?;
        }
    }

    Ok(())
}

pub fn read_in_names(in_names: &[PathBuf], name_context: &mut NameContext) -> BffCliResult<()> {
    for in_name in in_names {
        let f = File::open(in_name)?;
        let mut reader = BufReader::new(f);
        name_context.read(&mut reader)?;
    }

    Ok(())
}

pub fn write_names(
    out_names: &Path,
    names: Option<&[Name]>,
    name_context: &NameContext,
) -> BffCliResult<()> {
    let f = File::create(out_names)?;
    let mut writer = BufWriter::new(f);
    name_context.write(&mut writer, names)?;

    Ok(())
}

fn resolve_platform(bigfile_path: &Path, platform_override: Option<Platform>) -> Platform {
    platform_override.unwrap_or_else(|| {
        bigfile_path
            .extension()
            .and_then(|e| e.try_into().ok())
            .unwrap_or(Platform::PC)
    })
}

pub fn read_bigfile(
    bigfile_path: &Path,
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
    name_context: &NameContext,
) -> BffCliResult<BigFile> {
    let platform = resolve_platform(bigfile_path, platform_override);
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    Ok(BigFile::read_platform(
        &mut reader,
        platform,
        version_override,
        name_context,
    )?)
}

pub fn probe_bigfile_name_context(
    bigfile_path: &Path,
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
) -> BffCliResult<NameContext> {
    let platform = resolve_platform(bigfile_path, platform_override);
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    let name_type = BigFile::probe_name_type_platform(&mut reader, platform, version_override)?;
    Ok(NameContext::new(name_type))
}

pub fn read_artifacts(directory: &Path) -> BffCliResult<HashMap<OsString, Artifact>> {
    let mut artifacts = HashMap::new();

    for file in std::fs::read_dir(directory)? {
        let path = file?.path();
        if !path.is_file() || path.file_name() == Some(RESOURCE_JSON_FILE_NAME.as_ref()) {
            continue;
        }

        let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
            continue;
        };

        let Some(name) = path.file_stem() else {
            return Err(std::io::Error::other(format!(
                "artifact file has no stem: {}",
                path.display()
            ))
            .into());
        };
        let artifact_name = name.to_os_string();
        let artifact = match extension {
            "bin" => Artifact::Binary(std::fs::read(&path)?),
            "dds" => Artifact::Dds(std::fs::read(&path)?),
            "wav" => Artifact::Wav(std::fs::read(&path)?),
            "txt" => Artifact::Text(std::fs::read_to_string(&path)?),
            _ => continue,
        };
        artifacts.insert(artifact_name, artifact);
    }

    Ok(artifacts)
}

pub fn write_artifacts(
    directory: &Path,
    artifacts: impl IntoIterator<Item = (OsString, Artifact)>,
) -> BffCliResult<()> {
    for (name, artifact) in artifacts {
        let artifact_path = directory.join(name);
        match artifact {
            Artifact::Binary(bytes) => std::fs::write(artifact_path.with_extension("bin"), bytes)?,
            Artifact::Dds(bytes) => std::fs::write(artifact_path.with_extension("dds"), bytes)?,
            Artifact::Wav(bytes) => std::fs::write(artifact_path.with_extension("wav"), bytes)?,
            Artifact::Text(text) => std::fs::write(artifact_path.with_extension("txt"), text)?,
        }
    }

    Ok(())
}
