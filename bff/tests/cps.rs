use std::fs::File;
use std::io::Cursor;

use bff::Endian;
use bff::names::{NameContext, NameType};
use bff::tsc::{Cps, read_default_cps_names};
use binrw::io::BufReader;

use crate::path_helpers::resolve_repo_data_path;

#[datatest::data("tests/datasets/cps_read_write_read.yaml")]
#[test]
fn read_write_read(cps_path_str: String) {
    let cps_path = resolve_repo_data_path(&cps_path_str);
    let f = File::open(cps_path).unwrap();
    let mut reader = BufReader::new(f);
    let name_context = NameContext::new(NameType::BlackSheep32);
    read_default_cps_names(&name_context).unwrap();
    let cps = Cps::read(&mut reader, Endian::Little, &name_context).unwrap();
    let mut writer = Cursor::new(Vec::new());
    cps.write(&mut writer, Endian::Little, false, &name_context)
        .unwrap();

    let mut reader = Cursor::new(writer.into_inner());
    let name_context = NameContext::new(NameType::BlackSheep32);
    read_default_cps_names(&name_context).unwrap();
    let cps2 = Cps::read(&mut reader, Endian::Little, &name_context).unwrap();

    assert!(cps == cps2);
}
