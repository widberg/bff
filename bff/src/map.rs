use std::convert::TryFrom;
use std::fmt::Debug;

use binrw::BinRead;
use serde::Serialize;

use crate::dynarray::DynArray;

#[derive(Debug, BinRead, Serialize)]
#[serde(transparent)]
pub struct BffMap<KeyType, ValueType, SizeType = u32>(DynArray<(KeyType, ValueType), SizeType>)
where
    for<'a> KeyType: BinRead + Serialize + 'a,
    for<'a> <KeyType as BinRead>::Args<'a>: Clone + Default,

    for<'a> ValueType: BinRead + Serialize + 'a,
    for<'a> <ValueType as BinRead>::Args<'a>: Clone + Default,

    for<'a> (KeyType, ValueType): BinRead + Serialize + 'a,
    for<'a> <(KeyType, ValueType) as BinRead>::Args<'a>: Clone + Default,
    
    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>;
