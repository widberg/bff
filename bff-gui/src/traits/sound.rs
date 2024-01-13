use std::sync::Arc;

use super::export::Export;
use crate::artifact::Artifact;

impl Export for bff::class::sound::v1_291_03_06_pc::SoundV1_291_03_06PC {
    fn export(self) -> Artifact {
        Artifact::Sound {
            data: Arc::new(self.body.data),
            sample_rate: self.body.sample_rate,
            channels: match self.body.flags.stereo().value() {
                1 => 2,
                _ => 1,
            },
        }
    }
}

impl Export for bff::class::sound::v1_381_67_09_pc::SoundV1_381_67_09PC {
    fn export(self) -> Artifact {
        Artifact::Sound {
            data: Arc::new(self.body.data),
            sample_rate: self.link_header.sample_rate,
            channels: match self.link_header.flags.stereo().value() {
                1 => 2,
                _ => 1,
            },
        }
    }
}
