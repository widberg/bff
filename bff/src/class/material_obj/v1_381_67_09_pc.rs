use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffMap, DynArray, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(Serialize, Deserialize, JsonSchema)]
struct MaterialObjEntryV1_381_67_09PC {
    key: Name,
    value: DynArray<Name>,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct MaterialObjBodyV1_381_67_09PC {
    #[serde(with = "material_obj_entries")]
    #[schemars(with = "Vec<MaterialObjEntryV1_381_67_09PC>")]
    entries: BffMap<Name, DynArray<Name>>,
}

pub type MaterialObjV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, MaterialObjBodyV1_381_67_09PC>;

impl Export for MaterialObjV1_381_67_09PC {}
impl Import for MaterialObjV1_381_67_09PC {}

mod material_obj_entries {
    use indexmap::IndexMap;
    use serde::de::Error as _;
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::MaterialObjEntryV1_381_67_09PC;
    use crate::helpers::{BffMap, DynArray};
    use crate::names::Name;

    #[derive(Serialize)]
    struct MaterialObjEntryRefV1_381_67_09PC<'a> {
        key: &'a Name,
        value: &'a DynArray<Name>,
    }

    pub fn serialize<S>(
        entries: &BffMap<Name, DynArray<Name>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(entries.len()))?;
        for (key, value) in entries.iter() {
            seq.serialize_element(&MaterialObjEntryRefV1_381_67_09PC { key, value })?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BffMap<Name, DynArray<Name>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = Vec::<MaterialObjEntryV1_381_67_09PC>::deserialize(deserializer)?;
        let mut map = IndexMap::with_capacity(entries.len());
        for entry in entries {
            if map.insert(entry.key, entry.value).is_some() {
                return Err(D::Error::custom("duplicate material object entry key"));
            }
        }
        Ok(map.into())
    }
}
