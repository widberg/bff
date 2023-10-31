use std::collections::HashMap;

use bff::class::Class;
use bff::names::Name;

use crate::artifact::Artifact;

pub trait Export {
    fn export(self) -> Artifact;
}

pub trait RecursiveExport {
    fn export(self, resources: &HashMap<Name, Class>) -> Artifact;
    fn dependencies(&self) -> Vec<Name> {
        Vec::new()
    }
}
