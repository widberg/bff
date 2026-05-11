pub mod manifest;
pub mod platforms;
pub mod resource;
mod v1_06_63_02_pc;
mod v1_08_40_02_pc;
mod v1_2000_77_18_pc;
mod v1_2002_45_19_pc;
mod v1_22_pc;
mod v1_381_64_09_pc;
mod v2_07_pc;
mod v2_0_pc;
mod v2_128_52_19_pc;
mod v2_128_92_19_pc;
mod v2_256_38_19_pc;
pub mod versions;

use std::collections::HashMap;

use petgraph::Graph;

use crate::bigfile::manifest::Manifest;
use crate::bigfile::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::BigFileV1_06_63_02PC;
use crate::bigfile::v1_08_40_02_pc::BigFileV1_08_40_02PC;
use crate::bigfile::v1_22_pc::{
    BigFileV1_22PC,
    BigFileV1_22PCNoVersionTriple,
    BigFileV1_22PCNoVersionTripleBlackSheep,
};
use crate::bigfile::v1_381_64_09_pc::BigFileV1_381_64_09PC;
use crate::bigfile::v1_2000_77_18_pc::BigFileV1_2000_77_18PC;
use crate::bigfile::v1_2002_45_19_pc::BigFileV1_2002_45_19PC;
use crate::bigfile::v2_0_pc::BigFileV2_0PC;
use crate::bigfile::v2_07_pc::{BigFileV2_07PCPROTO, BigFileV2_07PCSHAUN};
use crate::bigfile::v2_128_52_19_pc::BigFileV2_128_52_19PC;
use crate::bigfile::v2_128_92_19_pc::BigFileV2_128_92_19PC;
use crate::bigfile::v2_256_38_19_pc::BigFileV2_256_38_19PC;
use crate::macros::bigfiles::bigfiles;
use crate::names::{Name, NameContext};
use crate::traits::ReferencedNames;

pub static DEFAULT_TAG: &str = "made with <3 by bff contributors (https://github.com/widberg/bff)";

pub type ResourceMap = HashMap<Name, Resource>;

#[derive(Debug, Eq, PartialEq)]
pub struct BigFile {
    manifest: Manifest,
    resources: ResourceMap,
}

impl BigFile {
    pub const fn new(manifest: Manifest, resources: ResourceMap) -> Self {
        Self {
            manifest,
            resources,
        }
    }

    pub const fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn resource_names(&self) -> impl ExactSizeIterator<Item = Name> + '_ {
        self.resources.keys().copied()
    }

    pub fn bff_resources(
        &self,
    ) -> impl ExactSizeIterator<Item = crate::bigfile::resource::BffResourceRef<'_>> + '_ {
        let platform = self.manifest.platform;
        let version = &self.manifest.version;
        self.resources
            .values()
            .map(move |resource| crate::bigfile::resource::BffResourceRef {
                platform,
                version,
                resource,
            })
    }

    pub fn bff_resource(&self, name: Name) -> Option<crate::bigfile::resource::BffResourceRef<'_>> {
        let platform = self.manifest.platform;
        let version = &self.manifest.version;
        self.resources
            .get(&name)
            .map(|resource| crate::bigfile::resource::BffResourceRef {
                platform,
                version,
                resource,
            })
    }

    pub fn reference_graph(&self, name_context: &NameContext) -> Graph<Name, ()> {
        let mut graph = Graph::with_capacity(self.resources.len(), 0);
        let mut node_ids = HashMap::new();
        for bff_resource in self.bff_resources() {
            let name = bff_resource.resource.name;
            let references = bff_resource
                .bff_class(name_context)
                .map(|bff_class| bff_class.class.referenced_names())
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

    pub fn probe_name_type_platform<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _platform: crate::bigfile::platforms::Platform,
        version_override: Option<&crate::bigfile::versions::Version>,
    ) -> crate::BffResult<crate::names::NameType> {
        use binrw::BinRead;

        let start = reader.stream_position()?;
        let version: crate::bigfile::versions::Version =
            crate::helpers::FixedStringNull::<256>::read_be(reader)?
                .as_str()
                .into();
        reader.seek(std::io::SeekFrom::Start(start))?;

        let version = version_override.cloned().unwrap_or(version);
        version.name_type()
    }
}

// TODO: All this type stuff is nonsense. Just have modules and re-check the bf
// version/platform there instead of using generics and binrw derive everywhere.
// Actually, I like the idea of having a monolithic backend and a
// BigFileBackendConfiguration or split read/write configs with lots of levers
// and knobs. We can define the config similar to what's below but the rest of
// the code should be a lot simpler and less repetitive.
bigfiles! {
    Kalisto(1, 75 | 73) | BlackSheep(1, _) => BigFileV1_22PCNoVersionTripleBlackSheep,
    Kalisto(1, _) => BigFileV1_22PCNoVersionTriple,
    BlackSheep(2, ..=7) | BlackSheep(2, 158..) => BigFileV2_07PCPROTO,
    BlackSheep(2, _) => BigFileV2_07PCSHAUN,
    Ubisoft { .. } => BigFileV2_0PC,
    AsoboLegacy(1, ..=80) => BigFileV1_22PC,
    AsoboLegacy(1, _) | Asobo(1, 1..=5 | 8, _, _) => BigFileV1_08_40_02PC,
    Asobo(1, 1..=380, _, _) => BigFileV1_06_63_02PC,
    Asobo(1, 381..=1999, _, _) => BigFileV1_381_64_09PC,
    Asobo(1, 0 | 2000..=2001, _, _) => BigFileV1_2000_77_18PC,
    Asobo(1, 2002.., _, _) => BigFileV1_2002_45_19PC,
    Asobo(2, 128, 92 | 18, _) => BigFileV2_128_92_19PC,
    Asobo(2, 256, 49, _) | Asobo(2, 128, 52, _) => BigFileV2_128_52_19PC,
    Asobo(2, 256, _, _) => BigFileV2_256_38_19PC,
}
