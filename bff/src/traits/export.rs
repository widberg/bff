use std::collections::HashMap;
use std::ffi::OsString;

use crate::BffResult;
use crate::error::Error;

pub enum Artifact {
    Binary(Vec<u8>),
    Text(String),
}

pub trait Export {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        Err(Error::UnimplementedImportExport)
    }
}

pub trait Import {
    fn import(&mut self, _artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        Err(Error::UnimplementedImportExport)
    }
}
