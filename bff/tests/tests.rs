#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Cursor;
    use std::path::PathBuf;

    use bff::bigfile::BigFile;
    use bff::bigfile::platforms::Platform;
    use bff::bigfile::resource::Resource;
    use bff::class::Class;
    use bff::traits::TryIntoVersionPlatform;
    use binrw::io::BufReader;

    #[datatest::data("../data/read.yaml")]
    #[test]
    fn read(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let _ = BigFile::read_platform(&mut reader, platform, &None).unwrap();
    }

    #[datatest::data("../data/roundtrip_resources.yaml")]
    #[test]
    fn roundtrip_resources(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let bigfile = BigFile::read_platform(&mut reader, platform, &None).unwrap();

        for resource in bigfile.resources.values() {
            let class: Class = resource
                .try_into_version_platform(bigfile.manifest.version.clone(), platform)
                .unwrap();

            let new_resource: Resource = (&class)
                .try_into_version_platform(bigfile.manifest.version.clone(), platform)
                .unwrap();

            assert_eq!(new_resource, *resource);
        }
    }

    #[datatest::data("../data/roundtrip_bigfiles.yaml")]
    #[test]
    fn roundtrip_bigfiles(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let data = fs::read(bigfile_path).unwrap();
        let mut reader = Cursor::new(&data);
        let bigfile = BigFile::read_platform(&mut reader, platform, &None).unwrap();
        let mut writer = Cursor::new(Vec::new());
        bigfile.write(&mut writer, None, &None, &None, None).unwrap();

        assert_eq!(data, writer.into_inner());
    }
}
