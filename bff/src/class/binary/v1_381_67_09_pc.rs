use crate::class::trivial_class::TrivialClass;
use bilge::prelude::12;
use bilge::prelude::20;
use bilge::prelude::4;

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(32)]
struct LookupDescription {
	horizon: u12,
	altitudes_index: u20,
}

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(8)]
struct AltitudePack {
	odd: u4,
	even: u4,
}

#[derive(BinRead, Debug, Serialize)]
struct AltitudesPacked {
	altitudes: [AltitudePack; 8],
}

#[derive(BinRead, Debug, Serialize)]
struct AltitudesUnpacked {
	altitudes: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct Internal {
	width: u32,
	height: u32,
	lookup_width = width / 4: u32,
	two: f32,
	negative_one: i32,
	denominator: f32,
	altitudes_packed_size: u32,
	altitudes_total_size: u32,
	#[br(count = altitudes_packed_size)]	altitudes_packed: Vec<AltitudesPacked>,
	#[br(count = ((altitudes_total_size - 1) * 4 - sizeof(altitudes_packed)) / 16)]	altitudes_unpacked: Vec<AltitudesUnpacked>,
	#[br(count = lookup_width * lookup_width)]	lookup: Vec<LookupDescription>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct BinaryBodyV1_381_67_09PC {
	data_size: u32,
	data: Internal,
}

pub type BinaryV1_381_67_09PC = TrivialClass<(), BinaryBodyV1_381_67_09PC>;