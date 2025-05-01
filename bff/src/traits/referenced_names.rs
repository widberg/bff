use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::{Range, RangeInclusive};

use impl_trait_for_tuples::impl_for_tuples;
use indexmap::IndexMap;

use crate::names::Name;

pub trait ReferencedNames {
    fn extend_referenced_names(&self, names: &mut HashSet<Name>);
    fn referenced_names(&self) -> HashSet<Name> {
        let mut names = HashSet::default();
        ReferencedNames::extend_referenced_names(self, &mut names);
        names
    }
}

impl<T: ReferencedNames, const N: usize> ReferencedNames for [T; N] {
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        for item in self {
            ReferencedNames::extend_referenced_names(item, names);
        }
    }
}

impl<T> ReferencedNames for Vec<T>
where
    T: ReferencedNames,
{
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        for item in self {
            ReferencedNames::extend_referenced_names(item, names);
        }
    }
}

impl ReferencedNames for Name {
    #[inline]
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        if !self.is_default() {
            names.insert(*self);
        }
    }
}

#[impl_for_tuples(1, 12)]
impl ReferencedNames for Tuple {
    #[inline]
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        for_tuples!( #( ReferencedNames::extend_referenced_names(&self.Tuple, names); )* );
    }
}

impl<T> ReferencedNames for PhantomData<T> {
    fn extend_referenced_names(&self, _: &mut HashSet<Name>) {}
}

impl<T> ReferencedNames for Option<T>
where
    T: ReferencedNames,
{
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        if let Some(item) = self {
            ReferencedNames::extend_referenced_names(item, names);
        }
    }
}

impl<T> ReferencedNames for Range<T> {
    fn extend_referenced_names(&self, _: &mut HashSet<Name>) {}
}

impl<T> ReferencedNames for RangeInclusive<T> {
    fn extend_referenced_names(&self, _: &mut HashSet<Name>) {}
}

impl<KeyType, ValueType> ReferencedNames for IndexMap<KeyType, ValueType>
where
    KeyType: ReferencedNames,
    ValueType: ReferencedNames,
{
    fn extend_referenced_names(&self, names: &mut HashSet<Name>) {
        for (k, v) in self.iter() {
            ReferencedNames::extend_referenced_names(k, names);
            ReferencedNames::extend_referenced_names(v, names);
        }
    }
}

macro_rules! impl_referenced_names {
    ($($t:ty),+) => {
        $(impl ReferencedNames for $t {
            #[inline]
            fn extend_referenced_names(&self, _: &mut HashSet<Name>) {
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
    String,
    usize,
    isize,
    char
);
