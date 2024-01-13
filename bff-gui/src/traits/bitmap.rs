use std::sync::Arc;

use super::export::Export;
use crate::artifact::Artifact;

impl Export for bff::class::bitmap::v1_291_03_06_pc::BitmapV1_291_03_06PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            is_dds: true,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_381_67_09_pc::BitmapV1_381_67_09PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            is_dds: true,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_06_63_02_pc::BitmapV1_06_63_02PC {
    fn export(self) -> Artifact {
        if let Some(dds) = self.body.dds {
            Artifact::Bitmap {
                is_dds: true,
                data: Arc::new(dds),
            }
        } else if let Some(data) = self.body.tex {
            Artifact::Bitmap {
                is_dds: false,
                data: Arc::new(data),
            }
        } else {
            Artifact::Bitmap {
                is_dds: false,
                data: Arc::new(Vec::new()),
            }
        }
    }
}
