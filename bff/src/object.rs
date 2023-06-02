use crate::name::Name;
use binrw::binread;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum BodySize {
    Uncompressed {
        size: u32,
    },
    Compressed {
        decompressed_size: u32,
        compressed_size: u32,
    },
}

#[binread]
#[br(stream = s)]
#[derive(Serialize, Debug)]
pub struct ObjectPtr {
    #[br(temp)]
    data_size: u32,
    link_header_size: u32,
    #[br(temp)]
    decompressed_size: u32,
    #[br(temp)]
    compressed_size: u32,
    #[br(calc =
        if data_size == link_header_size {
            None
        } else {
            Some(if compressed_size == 0 {
                BodySize::Uncompressed { size: decompressed_size }
            } else {
                BodySize::Compressed { decompressed_size, compressed_size }
            })
        }
    )]
    body_size: Option<BodySize>,
    class_name: Name,
    name: Name,
    #[br(try_calc = s.stream_position(), pad_after = data_size)]
    data_position: u64,
}

impl ObjectPtr {
    pub fn class_name(self: &Self) -> Name {
        self.class_name
    }

    pub fn name(self: &Self) -> Name {
        self.name
    }

    pub fn link_header_size(self: &Self) -> u32 {
        self.link_header_size
    }

    pub fn body_size(self: &Self) -> &Option<BodySize> {
        &self.body_size
    }
}
