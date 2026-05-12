use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::bff_resource::{BffResource, BffResourceHeader};
use crate::bigfile::versions::Version;
use crate::class::Class;
use crate::names::NameContext;
use crate::traits::ToResource;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BffClass {
    pub header: BffResourceHeader,
    pub class: Class,
}

impl BffClass {
    pub fn bff_resource(&self, name_context: &NameContext) -> BffResult<BffResource> {
        self.bff_resource_with_override(None, None, name_context)
    }

    pub fn bff_resource_with_override(
        &self,
        platform_override: Option<Platform>,
        version_override: Option<&Version>,
        name_context: &NameContext,
    ) -> BffResult<BffResource> {
        let platform = platform_override.unwrap_or(self.header.platform);
        let version = version_override.unwrap_or(&self.header.version);
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
