use binrw::BinRead;
use serde::Serialize;

use super::object::Object;

#[derive(BinRead, Serialize, Debug)]
#[br(import(data_count: u32))]
pub struct Data {
    #[br(count = data_count)]
    pub objects: Vec<Object>,
}
