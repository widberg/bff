use std::collections::HashMap;
use std::ffi::OsString;

use crate::BffResult;

pub enum Artifact {
    Binary(Vec<u8>),
    Text(String),
    Json(String),
}

pub trait Export {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        todo!()
    }
}

pub trait Import
where
    Self: Sized,
{
    fn import(_artifacts: &HashMap<OsString, Artifact>) -> BffResult<Self> {
        todo!()
    }
}
