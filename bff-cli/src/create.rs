use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::bigfile::platforms::Platform;
use bff::bigfile::versions::Version;
use bff::bigfile::BigFile;
use bff::BufReader;

use crate::error::BffCliResult;
use crate::extract::write_names;

pub fn create(
    directory: &Path,
    bigfile_path: &Path,
    out_names: &Option<PathBuf>,
    platform_override: &Option<Platform>,
    version_override: &Option<Version>,
) -> BffCliResult<()> {
    let manifest_path = directory.join("manifest.json");
    let manifest_reader = BufReader::new(File::open(manifest_path)?);
    let manifest = serde_json::from_reader(manifest_reader)?;

    let mut bigfile = BigFile {
        manifest,
        objects: Default::default(),
    };

    let resources_path = directory.join("resources");
    std::fs::create_dir_all(&resources_path)?;

    for file in std::fs::read_dir(resources_path)? {
        let path = file?.path();
        if path.is_file() {
            let mut file_reader = BufReader::new(File::open(&path)?);
            let resource = bigfile.read_bff_resource(&mut file_reader)?;
            bigfile.objects.insert(resource.name, resource);
        }
    }

    let mut bigfile_writer = BufWriter::new(File::create(bigfile_path)?);
    bigfile.write(
        &mut bigfile_writer,
        *platform_override,
        version_override,
        None,
    )?;

    if let Some(out_names) = out_names {
        write_names(out_names, &Some(bigfile.objects.keys().collect()))?;
    }

    Ok(())
}
