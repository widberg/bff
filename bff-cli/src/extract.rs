use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::BigFile;
use bff::bigfile::platforms::{Platform, try_platform_style_to_name_extension};
use bff::bigfile::resource::{BffClass, BffResourceHeader, Resource};
use bff::bigfile::versions::Version;
use bff::class::Class;
use bff::names::Name;
use bff::traits::{Artifact, Export, TryIntoVersionPlatform};
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::error::{BffCliError, BffCliResult};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ExportStrategy {
    #[value(alias("b"))]
    Binary,
    #[value(alias("r"))]
    Rich,
}

pub fn read_bigfile_names(bigfile_path: &Path) -> BffCliResult<()> {
    if let Some(extension) = bigfile_path.extension() {
        let name_extension =
            try_platform_style_to_name_extension(extension.try_into()?, extension.try_into()?)?;
        let in_name = bigfile_path.with_extension(name_extension);

        if let Ok(f) = File::open(in_name) {
            let mut reader = BufReader::new(f);
            bff::names::names().lock().unwrap().read(&mut reader)?;
        }
    }

    Ok(())
}

pub fn read_in_names(in_names: &Vec<PathBuf>) -> BffCliResult<()> {
    for in_name in in_names {
        let f = File::open(in_name)?;
        let mut reader = BufReader::new(f);
        bff::names::names().lock().unwrap().read(&mut reader)?;
    }

    Ok(())
}

pub fn write_names(out_names: &Path, names: &Option<Vec<&Name>>) -> BffCliResult<()> {
    let f = File::create(out_names)?;
    let mut writer = BufWriter::new(f);
    bff::names::names()
        .lock()
        .unwrap()
        .write(&mut writer, names)?;

    Ok(())
}

pub fn read_bigfile(
    bigfile_path: &Path,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<BigFile> {
    let platform = platform_override.unwrap_or_else(|| {
        bigfile_path
            .extension()
            .and_then(|e| e.try_into().ok())
            .unwrap_or(Platform::PC)
    });
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    Ok(BigFile::read_platform(
        &mut reader,
        platform,
        version_override,
    )?)
}

const INVALID_PATH_CHARS: [u8; 41] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 34, 42, 47, 58, 60, 62, 63, 92, 124,
];

fn clean_path(path: String) -> String {
    path.chars()
        .map(|v| {
            if INVALID_PATH_CHARS.contains(&(v as u8)) {
                '_'
            } else {
                v
            }
        })
        .collect()
}

fn dump_bff_resource(
    resources_path: &Path,
    bigfile: &BigFile,
    resource: &Resource,
) -> BffCliResult<()> {
    let name = clean_path(format!("{}", resource.name));
    let class_name = resource.class_name;
    let mut path = resources_path.join(format!("{}.{}", name, class_name));
    let mut i = 0;
    while path.exists() {
        path.set_file_name(format!("{}_{}.{}", name, i, class_name));
        i += 1;
    }
    let mut writer = BufWriter::new(File::create(path)?);
    bigfile.dump_bff_resource(resource, &mut writer)?;
    Ok(())
}

fn export_bff_resource(
    resources_path: &Path,
    bigfile: &BigFile,
    resource: &Resource,
) -> BffCliResult<()> {
    let platform = bigfile.manifest.platform;
    let version = bigfile.manifest.version.clone();
    let header = BffResourceHeader {
        platform,
        version: version.clone(),
    };

    let class: Class = resource.try_into_version_platform(version.clone(), platform)?;
    let bff_class = BffClass { header, class };

    let name = clean_path(format!("{}", resource.name));
    let class_name = resource.class_name;
    let mut directory = resources_path.join(format!("{}.{}.d", name, class_name));
    let mut i = 0;
    while directory.exists() {
        directory.set_file_name(format!("{}_{}.{}.d", name, i, class_name));
        i += 1;
    }

    std::fs::create_dir(&directory)?;

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
                Artifact::Dds(bytes) => std::fs::write(artifact_path.with_extension("dds"), bytes)?,
                Artifact::Text(text) => std::fs::write(artifact_path.with_extension("txt"), text)?,
            }
        }
    }

    Ok(())
}

pub fn extract(
    bigfile_path: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
    export_strategy: &ExportStrategy,
) -> BffCliResult<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading names");
    read_bigfile_names(bigfile_path)?;
    read_in_names(in_names)?;

    progress_bar.set_message("Reading BigFile");
    let bigfile = read_bigfile(bigfile_path, platform_override, version_override)?;

    progress_bar.set_message("Writing manifest");
    std::fs::create_dir(directory)?;

    let manifest_path = directory.join("manifest.json");
    let manifest_writer = BufWriter::new(File::create(manifest_path)?);
    serde_json::to_writer_pretty(manifest_writer, &bigfile.manifest)?;

    progress_bar.set_style(ProgressStyle::default_bar());
    progress_bar.set_length(bigfile.resources.len() as u64);

    let resources_path = directory.join("resources");
    std::fs::create_dir(&resources_path)?;

    bigfile
        .resources
        .values()
        .par_bridge()
        .try_for_each(|resource| {
            progress_bar.inc(1);
            if !matches!(*export_strategy, ExportStrategy::Rich)
                || export_bff_resource(&resources_path, &bigfile, resource).is_err()
            {
                dump_bff_resource(&resources_path, &bigfile, resource)?;
            }

            Ok::<(), BffCliError>(())
        })?;

    progress_bar.finish_and_clear();

    Ok(())
}
