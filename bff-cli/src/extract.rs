use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::BufReader;
use bff::bigfile::BigFile;
use bff::bigfile::platforms::{Platform, try_platform_style_to_name_extension};
use bff::bigfile::resource::BffResourceRef;
use bff::bigfile::versions::Version;
use bff::names::{Name, NameContext};
use bff::traits::{Artifact, Export};
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

pub fn read_bigfile(
    bigfile_path: &Path,
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
    name_context: &NameContext,
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
        name_context,
    )?)
}

pub fn probe_bigfile_name_context(
    bigfile_path: &Path,
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
) -> BffCliResult<NameContext> {
    let platform = platform_override.unwrap_or_else(|| {
        bigfile_path
            .extension()
            .and_then(|e| e.try_into().ok())
            .unwrap_or(Platform::PC)
    });
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    let name_type = BigFile::probe_name_type_platform(&mut reader, platform, version_override)?;
    Ok(NameContext::new(name_type))
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

fn strip_suffix_if_exists(s: String, suffix: &str) -> String {
    let mut s = s;
    if let Some(stripped) = s.strip_suffix(suffix) {
        s.truncate(stripped.len());
    }
    s
}

fn dump_bff_resource(
    resources_path: &Path,
    bff_resource: &BffResourceRef,
    name_context: &NameContext,
) -> BffCliResult<()> {
    let class_name = bff_resource
        .resource
        .class_name
        .with_context(name_context)
        .to_string();
    let name = clean_path(strip_suffix_if_exists(
        bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string(),
        &format!(".{}", class_name),
    ));
    let mut path = resources_path.join(format!("{}.{}", name, class_name));
    let mut i = 0;
    while path.exists() {
        path.set_file_name(format!("{}_{}.{}", name, i, class_name));
        i += 1;
    }
    let mut writer = BufWriter::new(File::create(path)?);
    bff_resource.write(&mut writer, name_context)?;
    Ok(())
}

fn export_bff_resource(
    resources_path: &Path,
    bff_resource: &BffResourceRef,
    name_context: &NameContext,
    rich_suffix: &str,
) -> BffCliResult<()> {
    let bff_class = bff_resource.bff_class(name_context)?;

    let class_name = bff_resource
        .resource
        .class_name
        .with_context(name_context)
        .to_string();
    let name = clean_path(strip_suffix_if_exists(
        bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string(),
        &format!(".{}", class_name),
    ));
    let mut directory = resources_path.join(format!("{}.{}{}", name, class_name, rich_suffix));
    let mut i = 0;
    while directory.exists() {
        directory.set_file_name(format!("{}_{}.{}{}", name, i, class_name, rich_suffix));
        i += 1;
    }

    std::fs::create_dir(&directory)?;

    let resource_serialized_path = directory.join("resource.json");
    let resource_serialized_writer = BufWriter::new(File::create(resource_serialized_path)?);
    bff::names::json::to_writer_pretty(resource_serialized_writer, &bff_class, name_context)?;

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

pub fn extract(
    bigfile_path: &Path,
    directory: &Path,
    in_names: &[PathBuf],
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
    export_strategy: ExportStrategy,
    rich_suffix: &str,
) -> BffCliResult<()> {
    let mut name_context =
        probe_bigfile_name_context(bigfile_path, platform_override, version_override)?;
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading names");
    read_bigfile_names(bigfile_path, &mut name_context)?;
    read_in_names(in_names, &mut name_context)?;

    progress_bar.set_message("Reading BigFile");
    let bigfile = read_bigfile(
        bigfile_path,
        platform_override,
        version_override,
        &name_context,
    )?;

    progress_bar.set_message("Writing manifest");
    std::fs::create_dir_all(directory)?;

    let manifest_path = directory.join("manifest.json");
    let manifest_writer = BufWriter::new(File::create(manifest_path)?);
    bff::names::json::to_writer_pretty(manifest_writer, bigfile.manifest(), &name_context)?;

    progress_bar.set_style(ProgressStyle::default_bar());
    progress_bar.set_length(bigfile.bff_resources().len() as u64);

    let resources_path = directory.join("resources");
    std::fs::create_dir(&resources_path)?;

    bigfile
        .bff_resources()
        .par_bridge()
        .try_for_each(|bff_resource| {
            progress_bar.inc(1);
            if !matches!(export_strategy, ExportStrategy::Rich)
                || export_bff_resource(&resources_path, &bff_resource, &name_context, rich_suffix)
                    .is_err()
            {
                dump_bff_resource(&resources_path, &bff_resource, &name_context)?;
            }

            Ok::<(), BffCliError>(())
        })?;

    progress_bar.finish_and_clear();

    Ok(())
}
