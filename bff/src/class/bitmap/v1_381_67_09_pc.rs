use crate::class::trivial_class::TrivialClass;

#[bitsize(16)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum BitmapClass {
	SINGLE = 0,
	CUBEMAP = 2,
}

#[bitsize(8)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum BmFormat {
	BM_MULTIPLE_BITMAPS = 0,
	BM_A8L8 = 7,
	BM_DXT1 = 14,
	BM_DXT5 = 16,
}

#[bitsize(8)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum BitmapClass2 {
	CUBEMAP2 = 0,
	SINGLE2 = 3,
}

#[bitsize(8)]
#[derive(BinRead, Debug, Serialize, FromBits)]
enum BmTransp {
	BM_NO_TRANSP = 0,
	BM_TRANSP_ONE = 1,
	BM_TRANSP = 2,
	BM_CUBEMAP = 255,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct Bitmap_LinkHeaderBodyV1_381_67_09PC {
	bitmap_class: BitmapClass,
	width: u32,
	height: u32,
	bitmap_data_size: u32,
	flags: u8,
	bitmap_type: u8,
	pad: u16,
	layer: f32,
	format0: BmFormat,
	mip_map_count: u8,
	four: u8,
	bitmap_class2: BitmapClass2,
	format1: BmFormat,
	transparency: BmTransp,
}

#[derive(BinRead, Debug, Serialize)]
struct Bitmap {
	//FIXME: inherits Bitmap_Z_LinkHeader
	#[br(count = data_size - link_header_size)]	data: Vec<u8>,
}

pub type BitmapV1_381_67_09PC = TrivialClass<(), Bitmap>;