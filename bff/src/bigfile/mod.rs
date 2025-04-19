pub mod manifest;
pub mod platforms;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_08_40_02_pc;
mod v1_2000_77_18_pc;
mod v1_2002_45_19_pc;
mod v1_22_pc;
mod v2_07_pc;
mod v2_0_pc;
mod v2_128_52_19_pc;
mod v2_128_92_19_pc;
mod v2_256_38_19_pc;
pub mod versions;

use std::collections::HashMap;

use petgraph::Graph;
use serde::Serialize;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::BigFileV1_06_63_02PC;
use crate::bigfile::v1_08_40_02_pc::BigFileV1_08_40_02PC;
use crate::bigfile::v1_22_pc::{
    BigFileV1_22PC,
    BigFileV1_22PCNoVersionTriple,
    BigFileV1_22PCNoVersionTripleBlackSheep,
};
use crate::bigfile::v1_2000_77_18_pc::BigFileV1_2000_77_18PC;
use crate::bigfile::v1_2002_45_19_pc::BigFileV1_2002_45_19PC;
use crate::bigfile::v2_0_pc::BigFileV2_0PC;
use crate::bigfile::v2_07_pc::{BigFileV2_07PCPROTO, BigFileV2_07PCSHAUN};
use crate::bigfile::v2_128_52_19_pc::BigFileV2_128_52_19PC;
use crate::bigfile::v2_128_92_19_pc::BigFileV2_128_92_19PC;
use crate::bigfile::v2_256_38_19_pc::BigFileV2_256_38_19PC;
use crate::class::Class;
use crate::macros::bigfiles::bigfiles;
use crate::names::Name;
use crate::traits::{ReferencedNames, TryIntoVersionPlatform};

pub static DEFAULT_TAG: &str = "made with <3 by bff contributors (https://github.com/widberg/bff)";

#[derive(Serialize, Debug)]
pub struct BigFile {
    #[serde(flatten)]
    pub manifest: Manifest,
    #[serde(skip)]
    pub objects: HashMap<Name, Resource>,
}

impl BigFile {
    pub fn reference_graph(&self) -> Graph<Name, ()> {
        let mut graph = Graph::new();
        let mut node_ids = HashMap::new();
        for (&name, resource) in &self.objects {
            let references =
                <&Resource as TryIntoVersionPlatform<Class>>::try_into_version_platform(
                    resource,
                    self.manifest.version.clone(),
                    self.manifest.platform,
                )
                .map(|class| class.referenced_names())
                .unwrap_or_default();
            let from_id = *node_ids.entry(name).or_insert_with(|| graph.add_node(name));
            for reference in references {
                let to_id = *node_ids
                    .entry(reference)
                    .or_insert_with(|| graph.add_node(reference));
                graph.add_edge(from_id, to_id, ());
            }
        }
        graph
    }
}

bigfiles! {
    (Kalisto(1, 75 | 73) | BlackSheep(1, _), _) => BigFileV1_22PCNoVersionTripleBlackSheep,
    (Kalisto(1, _), _) => BigFileV1_22PCNoVersionTriple,
    (BlackSheep(2, ..=7) | BlackSheep(2, 158..), _) => BigFileV2_07PCPROTO,
    (BlackSheep(2, _), _) => BigFileV2_07PCSHAUN,
    (Ubisoft { .. }, _) => BigFileV2_0PC,
    (AsoboLegacy(1, ..=80), _) => BigFileV1_22PC,
    (AsoboLegacy(1, _) | Asobo(1, 1..=5 | 8, _, _), _) => BigFileV1_08_40_02PC,
    (Asobo(1, 1..=1999, _, _), _) => BigFileV1_06_63_02PC,
    (Asobo(1, 0 | 2000..=2001, _, _), _) => BigFileV1_2000_77_18PC,
    (Asobo(1, 2002.., _, _), _) => BigFileV1_2002_45_19PC,
    (Asobo(2, 128, 92 | 18, _), _) => BigFileV2_128_92_19PC,
    (Asobo(2, 256, 49, _) | Asobo(2, 128, 52, _), _) => BigFileV2_128_52_19PC,
    (Asobo(2, 256, _, _), _) => BigFileV2_256_38_19PC,
}
