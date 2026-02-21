use std::fmt::Debug;
use std::marker::PhantomData;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames, JsonSchema)]
#[serde(transparent)]
#[br(bound(
    for<'a> InnerType: BinRead<Args<'a>: Clone> + 'a,
    for<'a> SizeType: BinRead<Args<'a>: Clone + Default>,
    usize: TryFrom<SizeType, Error: Debug>,
))]
#[bw(bound(
    for<'a> InnerType: BinWrite<Args<'a>: Clone + Default> + 'a,
    for<'a> SizeType: BinWrite<Args<'a>: Clone + Default> + TryFrom<usize, Error: Debug>,
))]
#[br(import_raw(inner: <InnerType as BinRead>::Args<'_>))]
pub struct DynArray<InnerType, SizeType = u32> {
    #[br(temp)]
    #[bw(calc = inner.len().try_into().unwrap())]
    count: SizeType,
    #[deref]
    #[deref_mut]
    #[br(args { count: count.try_into().unwrap(), inner })]
    pub inner: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<InnerType, SizeType> From<Vec<InnerType>> for DynArray<InnerType, SizeType> {
    fn from(value: Vec<InnerType>) -> Self {
        Self {
            inner: value,
            _phantom: PhantomData,
        }
    }
}
