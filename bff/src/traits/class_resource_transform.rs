use crate::BffResult;
use crate::bigfile::platforms::Platform;
use crate::bigfile::resource::Resource;
use crate::bigfile::versions::Version;
use crate::names::NameContext;

pub trait FromResource: Sized {
    fn from_resource(
        resource: &Resource,
        version: Version,
        platform: Platform,
        name_context: &NameContext,
    ) -> BffResult<Self>;
}

pub trait IntoResource {
    fn into_resource(
        &self,
        version: Version,
        platform: Platform,
        name_context: &NameContext,
    ) -> BffResult<Resource>;
}
