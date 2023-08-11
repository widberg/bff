use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
	//FIXME: inherits ResourceObject_Z
	flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct SurfaceDatasBodyV1_381_67_09PC {

}

pub type SurfaceDatasV1_381_67_09PC = TrivialClass<(), SurfaceDatasBodyV1_381_67_09PC>;