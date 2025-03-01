use std::sync::Arc;

use super::export::Export;
use crate::artifact::{Artifact, BitmapFormat};

impl Export for bff::class::bitmap::v1_291_03_06_pc::BitmapV1_291_03_06PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            format: BitmapFormat::Dds,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_381_67_09_pc::BitmapV1_381_67_09PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            format: BitmapFormat::Dds,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_06_63_02_pc::BitmapV1_06_63_02PC {
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
