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
#[br(bound(
    for<'a> InnerType: BinRead<Args<'a>: Default>,
    for<'a> ConditionType: BinRead<Args<'a>: Default> + One + Zero + Eq,
))]
#[bw(bound(
    for<'a> InnerType: BinWrite<Args<'a>: Clone + Default>,
    for<'a> ConditionType: BinWrite<Args<'a>: Clone + Default> + One + Zero + Eq,
))]
pub struct BffOption<InnerType, ConditionType = u8> {
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

impl<InnerType, ConditionType> Default for BffOption<InnerType, ConditionType> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _phantom: Default::default(),
        }
    }
}
