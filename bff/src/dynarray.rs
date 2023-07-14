use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binread, BinRead};
use derive_more::Deref;
use serde::Serialize;

#[binread]
#[derive(Debug, Serialize, Deref)]
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
    #[br(count = size)]
    data: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}
