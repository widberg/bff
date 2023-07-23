use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binread, BinRead};
use derive_more::{Deref, IntoIterator};
use serde::Serialize;

#[binread]
#[derive(Debug, Serialize, Deref, IntoIterator)]
#[serde(transparent)]
pub struct DynArray<InnerType, SizeType = u32>
where
    // This code is ugly but the pretty syntax isn't stable yet
    // https://github.com/rust-lang/rust/issues/52662
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
{
    #[br(temp)]
    size: SizeType,
    #[deref]
    #[into_iterator(owned, ref, ref_mut)]
    #[br(count = size)]
    data: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<InnerType, SizeType> From<Vec<InnerType>> for DynArray<InnerType, SizeType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
{
    fn from(vec: Vec<InnerType>) -> Self {
        Self {
            data: vec,
            _phantom: PhantomData,
        }
    }
}
