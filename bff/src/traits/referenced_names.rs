use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::{Range, RangeInclusive};

use impl_trait_for_tuples::impl_for_tuples;
use indexmap::IndexMap;

use crate::names::Name;

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

impl_referenced_names!(
    (),
    bool,
    f32,
    f64,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    String
);
