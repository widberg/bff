use std::fmt::Debug;
use std::marker::PhantomData;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use derive_more::Deref;
use num_traits::{One, Zero};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Serialize, Deref, Debug, Deserialize, ReferencedNames, JsonSchema)]
pub struct BffOption<
    InnerType: BinRead + BinWrite,
    ConditionType: BinRead + BinWrite + One + Zero + Eq = u8,
> where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
    for<'a> InnerType: BinWrite<Args<'a> = ()>,
    for<'a> ConditionType: BinWrite<Args<'a> = ()>,
{
    #[br(temp)]
    #[bw(calc = if inner.is_some() { ConditionType::one() } else { ConditionType::zero() })]
    condition: ConditionType,
    #[deref]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[br(if(condition != ConditionType::zero()))]
    inner: Option<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<ConditionType>,
}

impl<InnerType: BinRead + BinWrite, ConditionType: BinRead + BinWrite + One + Zero + Eq> Default
    for BffOption<InnerType, ConditionType>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
    for<'a> InnerType: BinWrite<Args<'a> = ()>,
    for<'a> ConditionType: BinWrite<Args<'a> = ()>,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _phantom: Default::default(),
        }
    }
}
