use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;

#[derive(BinRead, Debug, Serialize)]
pub struct Spline {
	//FIXME: inherits Object_Z
	points: DynArray<Vec3f>,
	spline_segments: DynArray<SplineSegment>,
	vec: Vec4f,
	length: f32,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &Spline))]
pub struct SplineGraphBodyV1_381_67_09PC {
	point_names: DynArray<u32>,
	point_datas: DynArray<DynArray<u8>>,
	spline_segment_datas: DynArray<DynArray<u8>>,
}

pub type SplineGraphV1_381_67_09PC = TrivialClass<(), SplineGraphBodyV1_381_67_09PC>;