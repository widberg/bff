use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::bigfile::BigFile;
use bff::platforms::Platform;
use bff::BufReader;

use crate::error::BffCliResult;

pub fn read_names(bigfile_path: &Path, in_names: &Vec<PathBuf>) -> BffCliResult<()> {
    // Read the associated name file if it exists
    if let Some(extension) = bigfile_path.extension() {
        if let Some(extension) = extension.to_str() {
            let mut extension = extension.to_string();
            extension.replace_range(..1, "N");
            let in_name = bigfile_path.with_extension(extension);

            if let Ok(f) = File::open(in_name) {
                let mut reader = BufReader::new(f);
                bff::names::names().lock().unwrap().read(&mut reader)?;
            }
        }
    }

    // Read the names from the input name files
    for in_name in in_names {
        let f = File::open(in_name)?;
        let mut reader = BufReader::new(f);
        bff::names::names().lock().unwrap().read(&mut reader)?;
    }

    Ok(())
}

pub fn write_names(out_names: &Option<PathBuf>) -> BffCliResult<()> {
    if let Some(out_name) = out_names {
        let f = File::create(out_name)?;
        let mut writer = BufWriter::new(f);
        bff::names::names().lock().unwrap().write(&mut writer)?;
    }

    Ok(())
}

pub fn read_bigfile(bigfile_path: &Path) -> BffCliResult<BigFile> {
    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    Ok(BigFile::read_platform(&mut reader, platform)?)
}

pub fn extract(
    bigfile_path: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    out_names: &Option<PathBuf>,
) -> BffCliResult<()> {
    read_names(bigfile_path, in_names)?;

    let bigfile = read_bigfile(bigfile_path)?;

    std::fs::create_dir(directory)?;

    let manifest_path = directory.join("manifest.json");
    let manifest_writer = BufWriter::new(File::create(manifest_path)?);
    serde_json::to_writer_pretty(manifest_writer, &bigfile.manifest)?;

    let resources_path = directory.join("resources");
    std::fs::create_dir(&resources_path)?;

    for resource in bigfile.objects.values() {
        let name = resource.name;
        let class_name = resource.class_name;
        let path = resources_path.join(format!("{}.{}", name, class_name));
        let mut writer = BufWriter::new(File::create(path)?);
        bigfile.dump_resource(resource, &mut writer)?;
    }

    write_names(out_names)?;

    Ok(())
}
