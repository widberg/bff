#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

#[cfg(test)]
mod tests {
    use mimalloc::MiMalloc;

    #[global_allocator]
    static GLOBAL: MiMalloc = MiMalloc;

    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Cursor;
    use std::path::PathBuf;

    use bff::Endian;
    use bff::bigfile::BigFile;
    use bff::bigfile::resource::{BffClass, BffResourceHeader, Resource};
    use bff::class::Class;
    use bff::names::{NameContext, NameType};
    use bff::traits::{Export, Import, TryIntoVersionPlatform};
    use bff::tsc::{
        Cps,
        mqfel_settings_bin_create,
        mqfel_settings_bin_extract,
        read_default_cps_names,
    };
    use binrw::io::BufReader;

    #[datatest::data("../data/read.yaml")]
    #[test]
    fn read(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = bigfile_path.extension().unwrap().try_into().unwrap();
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let name_context = NameContext::default();
        let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();

        for (i, resource) in bigfile.resources.values().enumerate() {
            let resource_name = resource.name.with_context(&name_context).to_string();
            let class_name = resource.class_name.with_context(&name_context).to_string();

            assert!(
                name_context.contains(&resource.class_name),
                "missing class name for resource {i} {resource_name}: {class_name}",
            );
        }
    }

    #[datatest::data("../data/roundtrip_resources.yaml")]
    #[test]
    fn roundtrip_resources(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = bigfile_path.extension().unwrap().try_into().unwrap();
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let name_context = NameContext::default();
        let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
        let version = bigfile.manifest.version.clone();

        for resource in bigfile.resources.values() {
            let class: Class = resource
                .try_into_version_platform(version.clone(), platform)
                .unwrap();
            let bff_class = BffClass {
                header: BffResourceHeader {
                    platform,
                    version: version.clone(),
                },
                class,
            };
            let resource_serialized =
                bff::names::json::to_string_pretty(&bff_class, &name_context).unwrap();
            let mut roundtripped_bff_class: BffClass = bff::names::json::from_reader(
                Cursor::new(resource_serialized.into_bytes()),
                &name_context,
            )
            .unwrap();
            let artifacts = bff_class.class.export().unwrap_or_else(|_| HashMap::new());
            let _ = roundtripped_bff_class.class.import(&artifacts);

            let new_resource: Resource = (&roundtripped_bff_class.class)
                .try_into_version_platform(version.clone(), platform)
                .unwrap();
            let resource_name = resource.name.with_context(&name_context).to_string();
            let class_name = resource.class_name.with_context(&name_context).to_string();

            assert!(new_resource == *resource, "{resource_name}.{class_name}");
        }
    }

    #[datatest::data("../data/roundtrip_bigfiles.yaml")]
    #[test]
    fn roundtrip_bigfiles(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = bigfile_path.extension().unwrap().try_into().unwrap();
        let data = fs::read(bigfile_path).unwrap();
        let mut reader = Cursor::new(&data);
        let name_context = NameContext::default();
        let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
        let mut writer = Cursor::new(Vec::new());
        bigfile
            .write(&mut writer, None, &None, &None, None, &name_context)
            .unwrap();

        assert!(data == writer.into_inner());
    }

    #[datatest::data("../data/read_write_read.yaml")]
    #[test]
    fn read_write_read(bigfile_path_str: String) {
        let bigfile_path = PathBuf::from(bigfile_path_str);
        let platform = bigfile_path.extension().unwrap().try_into().unwrap();
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let name_context = NameContext::default();
        let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();

        for (i, resource) in bigfile.resources.values().enumerate() {
            let resource_name = resource.name.with_context(&name_context).to_string();
            let class_name = resource.class_name.with_context(&name_context).to_string();

            assert!(
                name_context.contains(&resource.class_name),
                "missing class name for resource {i} {resource_name}: {class_name}",
            );
        }

        let mut writer = Cursor::new(Vec::new());
        bigfile
            .write(&mut writer, None, &None, &None, None, &name_context)
            .unwrap();
        let mut reader = Cursor::new(writer.into_inner());
        let name_context = NameContext::default();
        let bigfile2 = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
        assert!(bigfile == bigfile2);
    }

    #[datatest::data("../data/mqfel_settings_roundtrip.yaml")]
    #[test]
    fn mqfel_settings_roundtrip(settings_bin_path_str: String) {
        let settings_bin_path = PathBuf::from(settings_bin_path_str);
        let data = fs::read(settings_bin_path).unwrap();
        let extracted = mqfel_settings_bin_extract(Cursor::new(&data)).unwrap();

        let mut roundtrip_writer = Cursor::new(Vec::new());
        mqfel_settings_bin_create(&extracted, &mut roundtrip_writer).unwrap();

        let roundtripped =
            mqfel_settings_bin_extract(Cursor::new(roundtrip_writer.into_inner())).unwrap();
        assert_eq!(extracted, roundtripped);
    }

    #[datatest::data("../data/cps_roundtrip.yaml")]
    #[test]
    fn cps_roundtrip(cps_path_str: String) {
        let cps_path = PathBuf::from(&cps_path_str);
        let data = fs::read(cps_path).unwrap();
        let mut reader = Cursor::new(&data);
        let name_context = NameContext::default();
        name_context.set_name_type(NameType::BlackSheep32);
        read_default_cps_names(&name_context).unwrap();
        let cps = Cps::read(&mut reader, Endian::Little, &name_context).unwrap();
        let mut writer = Cursor::new(Vec::new());
        cps.write(&mut writer, Endian::Little, false, &name_context)
            .unwrap();

        assert!(data == writer.into_inner());
    }
}
