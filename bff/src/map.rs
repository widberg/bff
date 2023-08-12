use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use binrw::BinRead;
use derive_more::Deref;
use indexmap::IndexMap;
use serde::Serialize;

use crate::dynarray::DynArray;

#[derive(Debug, Serialize, BinRead, Deref)]
#[serde(transparent)]
pub struct BffMap<KeyType, ValueType, SizeType = u32>
where
    for<'a> KeyType: Hash + Eq + 'a,

    for<'a> ValueType: 'a,

    for<'a> (KeyType, ValueType): BinRead + Serialize + 'a,
    for<'a> <(KeyType, ValueType) as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    #[deref]
    #[br(map = |pairs: DynArray<(KeyType, ValueType), SizeType>| <DynArray<(KeyType, ValueType), SizeType> as Into<Vec<(KeyType, ValueType)>>>::into(pairs).into_iter().collect::<IndexMap<_, _>>())]
    map: IndexMap<KeyType, ValueType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<KeyType, ValueType, SizeType> From<IndexMap<KeyType, ValueType>>
    for BffMap<KeyType, ValueType, SizeType>
where
    for<'a> KeyType: Hash + Eq + 'a,

    for<'a> ValueType: 'a,

    for<'a> (KeyType, ValueType): BinRead + Serialize + 'a,
    for<'a> <(KeyType, ValueType) as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    fn from(map: IndexMap<KeyType, ValueType>) -> Self {
        Self {
            map,
            _phantom: PhantomData,
        }
    }
}
