use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::bigfile::platforms::{try_platform_style_to_name_extension, Platform};
use bff::bigfile::versions::Version;
use bff::bigfile::BigFile;
use bff::names::Name;
use bff::BufReader;

use crate::error::BffCliResult;

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

pub fn write_names(out_names: &Option<PathBuf>, names: &Option<Vec<&Name>>) -> BffCliResult<()> {
    if let Some(out_name) = out_names {
        let f = File::create(out_name)?;
        let mut writer = BufWriter::new(f);
        bff::names::names()
            .lock()
            .unwrap()
            .write(&mut writer, names)?;
    }

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

pub fn extract(
    bigfile_path: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    read_bigfile_names(bigfile_path)?;
    read_in_names(in_names)?;

    let bigfile = read_bigfile(bigfile_path, platform_override, version_override)?;

    std::fs::create_dir(directory)?;

    let manifest_path = directory.join("manifest.json");
    let manifest_writer = BufWriter::new(File::create(manifest_path)?);
    serde_json::to_writer_pretty(manifest_writer, &bigfile.manifest)?;

    let resources_path = directory.join("resources");
    std::fs::create_dir(&resources_path)?;

    for resource in bigfile.objects.values() {
        let name = clean_path(format!("{}", resource.name));
        let class_name = resource.class_name;
        let mut path = resources_path.join(format!("{}.{}", name, class_name));
        let mut i = 0;
        // TODO: Should check if the file contains the same object and overwrite it.
        // This will be easier once there is a bff header for the files.
        // Also need to think about a "--sync" option to apply names after an extract.
        while path.exists() {
            path.set_file_name(format!("{}_{}.{}", name, i, class_name));
            i += 1;
        }
        let mut writer = BufWriter::new(File::create(path)?);
        bigfile.dump_resource(resource, &mut writer)?;
    }

    Ok(())
}
