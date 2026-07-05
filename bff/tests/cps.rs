use std::fs;
use std::io::Cursor;

use bff::Endian;
use bff::names::{NameContext, NameType};
use bff::tsc::{Cps, read_default_cps_names};

use crate::path_helpers::resolve_repo_data_path;

#[datatest::data("tests/datasets/cps_roundtrip.yaml")]
#[test]
fn roundtrip(cps_path_str: String) {
    let cps_path = resolve_repo_data_path(&cps_path_str);
    let data = fs::read(cps_path).unwrap();
    let mut reader = Cursor::new(&data);
    let mut name_context = NameContext::new(NameType::BlackSheep32);
    read_default_cps_names(&mut name_context).unwrap();
    let cps = Cps::read(&mut reader, Endian::Little, &name_context).unwrap();

    let mut writer = Cursor::new(Vec::new());
    cps.write(&mut writer, Endian::Little, false, &mut name_context)
        .unwrap();

    assert!(data == writer.into_inner());
}
