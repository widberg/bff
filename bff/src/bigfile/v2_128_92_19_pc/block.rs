use binrw::BinRead;

use super::resource::Resource;

#[derive(BinRead, Debug)]
#[br(import(data_count: u32))]
pub struct Data {
    #[br(count = data_count, align_after = 2048)]
    pub resources: Vec<Resource>,
}
