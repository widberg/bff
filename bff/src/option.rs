use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binread, BinRead};
use derive_more::Deref;
use num_traits::Zero;
use serde::Serialize;

#[binread]
#[derive(Debug, Serialize, Deref)]
#[serde(transparent)]
pub struct Option<InnerType, ConditionType = u8>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    ConditionType: BinRead + Debug + Copy + PartialEq + Zero,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
    usize: TryFrom<ConditionType>,
{
    #[br(temp)]
    condition: ConditionType,
    #[deref]
    #[br(if(condition != ConditionType::zero()))]
    data: std::option::Option<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<ConditionType>,
}

impl<InnerType, ConditionType> From<std::option::Option<InnerType>>
    for Option<InnerType, ConditionType>
where
    for<'a> InnerType: BinRead + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,

    ConditionType: BinRead + Debug + Copy + PartialEq + Zero,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
    usize: TryFrom<ConditionType>,
{
    fn from(opt: std::option::Option<InnerType>) -> Self {
        Self {
            data: opt,
            _phantom: PhantomData,
        }
    }
}
