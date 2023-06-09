use serde::Serialize;

use crate::name::Name;
use crate::object::Object;
use crate::platforms::Platform;
use crate::versions::Version;

pub trait TryIntoVersionPlatform<T>: Sized {
    type Error;

    fn try_into_version_platform(
        self,
        version: Version,
        platform: Platform,
    ) -> Result<T, Self::Error>;
}

pub trait TryFromVersionPlatform<T>: Sized {
    type Error;
    fn try_from_version_platform(
        value: T,
        version: Version,
        platform: Platform,
    ) -> Result<Self, Self::Error>;
}

impl<T, U> TryIntoVersionPlatform<U> for T
where
    U: TryFromVersionPlatform<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_into_version_platform(
        self,
        version: Version,
        platform: Platform,
    ) -> Result<U, U::Error> {
        U::try_from_version_platform(self, version, platform)
    }
}

pub trait ShadowClass: Sized + Serialize
where
    for<'a> &'a Object: TryIntoVersionPlatform<Self>,
{
    const NAME: Name;
}
