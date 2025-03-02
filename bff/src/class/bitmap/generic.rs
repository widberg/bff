use crate::class::trivial_class::TrivialClass;

pub struct BitmapHeaderGeneric {
    pub width: u32,
    pub height: u32,
    pub mipmap_count: u8,
}

pub struct BitmapBodyGeneric {
    pub bitmap_header: BitmapHeaderGeneric,
    pub data: Vec<u8>,
}

pub type BitmapGeneric = TrivialClass<(), BitmapBodyGeneric>;
