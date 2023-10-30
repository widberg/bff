use std::{fs::File, io::Write, path::PathBuf};

use bff::{bigfile::BigFile, class::Class, names::Name, traits::TryIntoVersionPlatform};

pub fn write_class_json(path: &PathBuf, bigfile: &BigFile, resource_name: &Name) {
    File::create(path)
        .unwrap()
        .write_all(
            serde_json::to_string_pretty::<Class>(
                &bigfile
                    .objects
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
