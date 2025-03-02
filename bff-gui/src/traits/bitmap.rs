use std::sync::Arc;

use super::export::Export;
use crate::artifact::{Artifact, BitmapFormat};

impl Export for bff::class::bitmap::generic::BitmapGeneric {
    fn export(self) -> Artifact {
        let magic = &self.body.data[..4];
        let format = match magic {
            &[0x44, 0x44, 0x53, 0x20] => BitmapFormat::Dds,
            _ => BitmapFormat::Raw,
        };
        Artifact::Bitmap {
            format,
            data: Arc::new(self.body.data),
        }
    }
}
