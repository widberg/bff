use std::{convert::TryFrom, fmt::Display};
use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binread, BinRead, NamedArgs};
use derive_more::Deref;
use serde::Serialize;

#[derive(Clone, NamedArgs, Default)]
pub struct DynArrayArgs<Inner> {
    inner: Inner,
}

#[binread]
#[derive(Debug, Serialize, Deref)]
#[serde(transparent)]
#[br(import_raw(args: DynArrayArgs<<InnerType as BinRead>::Args<'_>>))]
pub struct DynArray<InnerType, SizeType = u32>
where
    // This code is ugly but the pretty syntax isn't stable yet
    // https://github.com/rust-lang/rust/issues/52662
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    #[br(temp, try_map = |count: SizeType| count.try_into())]
    count: usize,
    #[deref]
    #[br(args { count, inner: args.inner })]
    data: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<InnerType, SizeType> From<DynArray<InnerType, SizeType>> for Vec<InnerType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    fn from(dynarray: DynArray<InnerType, SizeType>) -> Self {
        dynarray.data
    }
}

impl<InnerType, SizeType> From<Vec<InnerType>> for DynArray<InnerType, SizeType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    for<'a> <usize as TryFrom<SizeType>>::Error: Debug + Display + Send + Sync + 'a,
{
    fn from(vec: Vec<InnerType>) -> Self {
        Self {
            data: vec,
            _phantom: PhantomData,
        }
    }
}
