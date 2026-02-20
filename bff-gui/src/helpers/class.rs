use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::{Name, NameContext};
use bff::traits::TryIntoVersionPlatform;

pub fn write_class_json(
    path: &PathBuf,
    bigfile: &BigFile,
    resource_name: &Name,
    name_context: &NameContext,
) {
    File::create(path)
        .unwrap()
        .write_all(
            bff::names::json::to_string_pretty::<Class>(
                &bigfile
                    .resources
                    .get(resource_name)
                    .unwrap()
                    .try_into_version_platform(
                        bigfile.manifest.version.clone(),
                        bigfile.manifest.platform,
                    )
                    .unwrap(),
                name_context,
            )
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
}
