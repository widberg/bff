use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use binrw::{BinRead, NamedArgs};
use derive_more::{Deref, From, Into};
use indexmap::IndexMap;
use serde::Serialize;

use crate::dynarray::DynArray;

#[derive(Clone, Default, NamedArgs)]
pub struct BffMapArgs<KeyArgs, ValueArgs> {
    key: KeyArgs,
    value: ValueArgs,
}

#[derive(BinRead, Clone, Debug, Serialize, From, Into)]
#[br(import_raw(args: BffMapArgs<<KeyType as BinRead>::Args<'_>, <ValueType as BinRead>::Args<'_>>))]
struct Pair<KeyType, ValueType>(
    #[br(args_raw = args.key)] KeyType,
    #[br(args_raw = args.value)] ValueType)
where
    for<'a> KeyType: BinRead + Serialize + Hash + Eq + 'a,
    for<'a> <KeyType as BinRead>::Args<'a>: Clone + Default,

    for<'a> ValueType: BinRead + Serialize + 'a,
    for<'a> <ValueType as BinRead>::Args<'a>: Clone + Default;

#[derive(Debug, Serialize, BinRead, Deref)]
#[serde(transparent)]
pub struct BffMap<KeyType, ValueType, SizeType = u32>
where
    for<'a> KeyType: BinRead + Serialize + Hash + Eq + 'a,
    for<'a> <KeyType as BinRead>::Args<'a>: Clone + Default,

    for<'a> ValueType: BinRead + Serialize + 'a,
    for<'a> <ValueType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    #[deref]
    #[br(map = |pairs: DynArray<Pair<KeyType, ValueType>, SizeType>| <DynArray<Pair<KeyType, ValueType>, SizeType> as Into<Vec<Pair<KeyType, ValueType>>>>::into(pairs).into_iter().map(|pair| <Pair<KeyType, ValueType> as Into<(KeyType, ValueType)>>::into(pair)).collect::<IndexMap<_, _>>())]
    map: IndexMap<KeyType, ValueType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<KeyType, ValueType, SizeType> From<IndexMap<KeyType, ValueType>>
    for BffMap<KeyType, ValueType, SizeType>
where
    for<'a> KeyType: BinRead + Serialize + Hash + Eq + 'a,
    for<'a> <KeyType as BinRead>::Args<'a>: Clone + Default,

    for<'a> ValueType: BinRead + Serialize + 'a,
    for<'a> <ValueType as BinRead>::Args<'a>: Clone + Default,

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
