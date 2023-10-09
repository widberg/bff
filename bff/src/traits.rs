use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;

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

impl ReferencedNames for Name {
    fn names(&self) -> HashSet<Name> {
        let mut names = HashSet::new();
        names.insert(*self);
        names
    }
}

impl ReferencedNames for () {
    fn names(&self) -> HashSet<Name> {
        HashSet::new()
    }
}

impl<T> ReferencedNames for (T,)
where
    T: ReferencedNames,
{
    fn names(&self) -> HashSet<Name> {
        self.0.names()
    }
}

impl<T, U> ReferencedNames for (T, U)
where
    T: ReferencedNames,
    U: ReferencedNames,
{
    fn names(&self) -> HashSet<Name> {
        let mut names = self.0.names();
        names.extend(&self.1.names());
        names
    }
}

impl<T, U, V> ReferencedNames for (T, U, V)
where
    T: ReferencedNames,
    U: ReferencedNames,
    V: ReferencedNames,
{
    fn names(&self) -> HashSet<Name> {
        let mut names = self.0.names();
        names.extend(&self.1.names());
        names.extend(&self.2.names());
        names
    }
}

impl<T> ReferencedNames for PhantomData<T> {
    fn names(&self) -> HashSet<Name> {
        HashSet::new()
    }
}

impl<T> ReferencedNames for Option<T>
where
    T: ReferencedNames,
{
    fn names(&self) -> HashSet<Name> {
        self.as_ref()
            .map(ReferencedNames::names)
            .unwrap_or_default()
    }
}
