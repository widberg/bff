use crate::class::trivial_class::TrivialClass;
use bilge::prelude::1;
use bilge::prelude::13;

#[derive(BinRead, DebugBits, Serialize, FromBits)]
#[bitsize(16)]
struct SoundFlags {
	PAUSED: u1,
	LOOPING: u1,
	STEREO: u1,
	#[br(temp)]	padding: u13,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
	//FIXME: inherits BaseObject_Z
	link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct Sound_LinkHeaderBodyV1_381_67_09PC {
	sample_rate: u32,
	sound_data_size: u32,
	flags: SoundFlags,
}

#[derive(BinRead, Debug, Serialize)]
struct Sound {
	//FIXME: inherits Sound_Z_LinkHeader
	#[br(count = sound_data_size)]	data: Vec<u8>,
}

pub type SoundV1_381_67_09PC = TrivialClass<(), Sound>;