use std::collections::HashSet;
use std::io::{Read, Seek, Write};

use crate::bigfile::BigFile;
use crate::names::Name;
use crate::platforms::Platform;
use crate::versions::Version;
use crate::BffResult;

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

pub trait NamedClass {
    const NAME: Name;
    const NAME_STR: &'static str;
}

pub trait BigFileRead {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile>;
}

pub trait BigFileWrite {
    fn write<W: Write + Seek>(bigfile: &BigFile, writer: &mut W) -> BffResult<()>;
}

pub trait ReferencedNames {
    fn names(&self) -> HashSet<Name>;
}

impl<T: ReferencedNames, const N: usize> ReferencedNames for [T; N] {
    fn names(&self) -> HashSet<Name> {
        self.iter().flat_map(ReferencedNames::names).collect()
    }
}

impl<T: ReferencedNames> ReferencedNames for Vec<T> {
    fn names(&self) -> HashSet<Name> {
        self.iter().flat_map(ReferencedNames::names).collect()
    }
}
