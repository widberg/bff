#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::sync::Arc;

use three_d::CpuModel;

#[cfg(not(target_arch = "wasm32"))]
use crate::error::BffGuiResult;

pub enum BitmapFormat {
    Dds,
    Raw,
}

pub enum Artifact {
    Bitmap {
        format: BitmapFormat,
        data: Arc<Vec<u8>>,
    },
    Sound {
        data: Arc<[u8]>,
    },
    Mesh(Arc<CpuModel>),
    Skin(Arc<CpuModel>),
}

#[cfg(not(target_arch = "wasm32"))]
impl Artifact {
    //TODO: write to impl Write
    pub fn save(&self, path: &PathBuf) -> BffGuiResult<()> {
        match *self {
            Self::Bitmap {
                format: _,
                ref data,
            } => {
                let mut file = File::create(path)?;
                file.write_all(data)?;
                Ok(())
            }
            Self::Sound { ref data } => {
                let mut file = File::create(path)?;
                file.write_all(data.as_ref())?;
                Ok(())
            }
            Self::Mesh(_) => todo!(),
            Self::Skin(_) => todo!(),
        }
    }
}
