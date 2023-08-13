use std::path::PathBuf;

use crate::error::GuiError;

pub trait Export {
    fn export(&self, export_path: &PathBuf, name: u32) -> Result<String, GuiError>;
}
