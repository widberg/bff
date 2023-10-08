use std::path::PathBuf;

use bff::names::Name;

use crate::error::BffGuiResult;

pub trait Export {
    fn export(&self, export_path: &PathBuf, name: Name) -> BffGuiResult<String>;
}
