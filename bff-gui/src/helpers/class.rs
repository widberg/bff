use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;

pub fn write_class_json(path: &PathBuf, bigfile: &BigFile, resource_name: &Name) {
    File::create(path)
        .unwrap()
        .write_all(
            serde_json::to_string_pretty::<Class>(
                &bigfile
                    .resources
                    .get(resource_name)
                    .unwrap()
                    .try_into_version_platform(
                        bigfile.manifest.version.clone(),
                        bigfile.manifest.platform,
                    )
                    .unwrap(),
            )
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
}
