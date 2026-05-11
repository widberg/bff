use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Cursor;

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::{BffClass, Resource};
use bff::names::NameContext;
use bff::traits::{Export, Import, ToResource};
use binrw::io::BufReader;

use crate::path_helpers::resolve_bigfile_path;

fn assert_no_missing_class_names(bigfile: &BigFile, name_context: &NameContext) {
    for (i, bff_resource) in bigfile.bff_resources().enumerate() {
        let resource_name = bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string();
        let class_name = bff_resource
            .resource
            .class_name
            .with_context(name_context)
            .to_string();

        assert!(
            name_context.contains(bff_resource.resource.class_name),
            "missing class name for resource {i} {resource_name}: {class_name}",
        );
    }
}

fn probe_name_context<R: std::io::Read + std::io::Seek>(
    reader: &mut R,
    platform: Platform,
) -> NameContext {
    let name_type = BigFile::probe_name_type_platform(reader, platform, &None).unwrap();
    NameContext::new(name_type)
}

#[datatest::data("tests/datasets/bigfile_read.yaml")]
#[test]
fn read(bigfile_path_str: String) {
    let bigfile_path = resolve_bigfile_path(&bigfile_path_str);
    let platform = bigfile_path.extension().unwrap().try_into().unwrap();
    let f = File::open(bigfile_path).unwrap();
    let mut reader = BufReader::new(f);
    let name_context = probe_name_context(&mut reader, platform);
    let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
    assert_no_missing_class_names(&bigfile, &name_context);
}

#[datatest::data("tests/datasets/bigfile_roundtrip_resources.yaml")]
#[test]
fn roundtrip_resources(bigfile_path_str: String) {
    let bigfile_path = resolve_bigfile_path(&bigfile_path_str);
    let platform = bigfile_path.extension().unwrap().try_into().unwrap();
    let f = File::open(bigfile_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut name_context = probe_name_context(&mut reader, platform);
    let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
    let version = &bigfile.manifest().version;

    for bff_resource in bigfile.bff_resources() {
        let bff_class = bff_resource.bff_class(&name_context).unwrap();
        let resource_serialized =
            bff::names::json::to_string_pretty(&bff_class, &name_context).unwrap();
        let mut roundtripped_bff_class: BffClass = bff::names::json::from_reader(
            Cursor::new(resource_serialized.into_bytes()),
            &mut name_context,
        )
        .unwrap();
        let artifacts = bff_class.class.export().unwrap_or_else(|_| HashMap::new());
        let _ = roundtripped_bff_class.class.import(&artifacts);

        let new_resource: Resource = roundtripped_bff_class
            .class
            .to_resource(version, platform, &name_context)
            .unwrap();
        let resource_name = bff_resource
            .resource
            .name
            .with_context(&name_context)
            .to_string();
        let class_name = bff_resource
            .resource
            .class_name
            .with_context(&name_context)
            .to_string();

        assert!(
            new_resource == *bff_resource.resource,
            "{resource_name}.{class_name}"
        );
    }
}

#[datatest::data("tests/datasets/bigfile_roundtrip.yaml")]
#[test]
fn roundtrip(bigfile_path_str: String) {
    let bigfile_path = resolve_bigfile_path(&bigfile_path_str);
    let platform = bigfile_path.extension().unwrap().try_into().unwrap();
    let data = fs::read(bigfile_path).unwrap();
    let mut reader = Cursor::new(&data);
    let name_context = probe_name_context(&mut reader, platform);
    let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
    assert_no_missing_class_names(&bigfile, &name_context);

    let mut writer = Cursor::new(Vec::new());
    bigfile
        .write(&mut writer, None, &None, &None, None, &name_context)
        .unwrap();

    assert!(data == writer.into_inner());
}

#[datatest::data("tests/datasets/bigfile_read_write_read.yaml")]
#[test]
fn read_write_read(bigfile_path_str: String) {
    let bigfile_path = resolve_bigfile_path(&bigfile_path_str);
    let platform = bigfile_path.extension().unwrap().try_into().unwrap();
    let f = File::open(bigfile_path).unwrap();
    let mut reader = BufReader::new(f);
    let name_context = probe_name_context(&mut reader, platform);
    let bigfile = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
    assert_no_missing_class_names(&bigfile, &name_context);

    let mut writer = Cursor::new(Vec::new());
    bigfile
        .write(&mut writer, None, &None, &None, None, &name_context)
        .unwrap();
    let mut reader = Cursor::new(writer.into_inner());
    let name_context = probe_name_context(&mut reader, platform);
    let bigfile2 = BigFile::read_platform(&mut reader, platform, &None, &name_context).unwrap();
    assert!(bigfile == bigfile2);
}
