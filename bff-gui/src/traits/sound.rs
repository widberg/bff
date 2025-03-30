use std::sync::Arc;

use super::export::Export;
use crate::artifact::Artifact;

impl Export for bff::class::sound::generic::SoundGeneric {
    fn export(self) -> Artifact {
        Artifact::Sound {
            data: Arc::new(self.body.data),
            sample_rate: self.link_header.as_ref().unwrap().sample_rate,
            channels: match self.link_header.as_ref().unwrap().flags.stereo().value() {
                1 => 2,
                _ => 1,
            },
        }
    }
}
