use bff::names::Name;

use crate::error::BffGuiResult;
use crate::PreviewData;

pub trait Export {
    fn export(&self, name: Name) -> BffGuiResult<PreviewData>;
}
