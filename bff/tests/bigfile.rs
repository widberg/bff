#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use bff::bigfile::BigFile;
    use bff::class::Class;
    use bff::object::Object;
    use bff::platforms::Platform;
    use bff::traits::{TryFromVersionPlatform, TryIntoVersionPlatform};
    use binrw::io::BufReader;
    use test_generator::test_resources;

    #[test_resources("data/bigfiles/**/*.*")]
    fn read(bigfile_path_str: &str) {
        let mut bigfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bigfile_path.pop();
        bigfile_path.push(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let _ = BigFile::read_platform(&mut reader, platform).unwrap();
    }

    #[test_resources("data/bigfiles/FUEL/PC_US/v1_381_67_09/**/*.DPC")]
    fn read_objects_fuel(bigfile_path_str: &str) {
        let mut bigfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bigfile_path.pop();
        bigfile_path.push(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let bigfile = BigFile::read_platform(&mut reader, platform).unwrap();

        for object in bigfile.objects.values() {
            let class: Class = object
                .try_into_version_platform(bigfile.manifest.version.clone(), platform)
                .unwrap();

            let _: Object = (&class)
                .try_into_version_platform(bigfile.manifest.version.clone(), platform)
                .unwrap();
        }
    }
}
