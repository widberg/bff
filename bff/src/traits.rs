use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;
use std::ops::{Range, RangeInclusive};

use impl_trait_for_tuples::impl_for_tuples;
use indexmap::IndexMap;

use crate::bigfile::BigFile;
use crate::names::{Name, NameType};
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

pub trait NamedClass<N> {
    const NAME: N;
}

pub trait BigFileIo {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile>;

    fn write<W: Write + Seek>(bigfile: &BigFile, writer: &mut W) -> BffResult<()>;

    fn name_type(version: Version, platform: Platform) -> NameType;
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

#[impl_for_tuples(1, 12)]
impl ReferencedNames for Tuple {
    fn names(&self) -> HashSet<Name> {
        let mut names = HashSet::new();
        for_tuples!( #( names.extend(&self.Tuple.names()); )* );
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

impl<T> ReferencedNames for Range<T> {
    fn names(&self) -> HashSet<Name> {
        HashSet::new()
    }
}

impl<T> ReferencedNames for RangeInclusive<T> {
    fn names(&self) -> HashSet<Name> {
        HashSet::new()
    }
}

impl<KeyType, ValueType> ReferencedNames for IndexMap<KeyType, ValueType>
where
    KeyType: ReferencedNames,
    ValueType: ReferencedNames,
{
    fn names(&self) -> HashSet<Name> {
        let mut names = HashSet::new();
        for (k, v) in self.iter() {
            names.extend(k.names());
            names.extend(v.names());
        }
        names
    }
}

macro_rules! impl_referenced_names {
    ($($t:ty),+) => {
        $(impl ReferencedNames for $t {
            fn names(&self) -> HashSet<Name> {
                HashSet::new()
            }
        })+
    }
}

impl_referenced_names!((), bool, f32, f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, String);

// this should be const https://github.com/rust-lang/rust/issues/67792
pub trait NameHashFunction {
    type Target;
    fn hash(bytes: &[u8]) -> Self::Target;
    fn hash_options(bytes: &[u8], starting: Self::Target) -> Self::Target;
}
