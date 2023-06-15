use binrw::{binread, BinRead};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

#[binread]
#[derive(Debug)]
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
    #[br(count = size)]
    data: Vec<InnerType>,
    _phantom: PhantomData<SizeType>,
}

impl<InnerType, SizeType> Serialize for DynArray<InnerType, SizeType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<InnerType, SizeType> Deref for DynArray<InnerType, SizeType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
{
    type Target = Vec<InnerType>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
