use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::bigfile::resource::bff_resource::{BffResource, BffResourceHeader};
use crate::class::Class;
use crate::names::NameContext;
use crate::traits::ToResource as _;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BffClass {
    pub header: BffResourceHeader,
    pub class: Class,
}

impl BffClass {
    pub fn bff_resource(&self, name_context: &NameContext) -> BffResult<BffResource> {
        let platform = self.header.platform;
        let version = &self.header.version;
        let resource = self.class.to_resource(version, platform, name_context)?;
        Ok(BffResource {
            header: BffResourceHeader {
                platform,
                version: version.clone(),
            },
            resource,
        })
    }
}
