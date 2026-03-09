use std::fs;
use std::io::Cursor;

use bff::tsc::{mqfel_settings_bin_create, mqfel_settings_bin_extract};

use crate::path_helpers::resolve_repo_data_path;

#[datatest::data("tests/datasets/mqfel_settings_read_write_read.yaml")]
#[test]
fn read_write_read(settings_bin_path_str: String) {
    let settings_bin_path = resolve_repo_data_path(&settings_bin_path_str);
    let data = fs::read(settings_bin_path).unwrap();
    let extracted = mqfel_settings_bin_extract(Cursor::new(&data)).unwrap();

    let mut roundtrip_writer = Cursor::new(Vec::new());
    mqfel_settings_bin_create(&extracted, &mut roundtrip_writer).unwrap();

    let roundtripped =
        mqfel_settings_bin_extract(Cursor::new(roundtrip_writer.into_inner())).unwrap();
    assert!(extracted == roundtripped);
}
