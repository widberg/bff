use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{BinRead, BinResult, BinWrite};
use derive_more::Deref;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use crate::Endian;

#[derive(Serialize, Deref, Debug, Deserialize)]
pub struct BffOption<InnerType, ConditionType = u8> {
    #[deref]
    #[serde(skip_serializing_if = "Option::is_none")]
    inner: Option<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<ConditionType>,
}

impl<InnerType: BinRead, ConditionType: BinRead + Zero + Eq> BinRead
    for BffOption<InnerType, ConditionType>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Default,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let condition = ConditionType::read_options(reader, endian, <_>::default())?;
        let inner = if condition != ConditionType::zero() {
            Some(InnerType::read_options(reader, endian, <_>::default())?)
        } else {
            None
        };
        Ok(Self {
            inner,
            _phantom: PhantomData,
        })
    }
}

impl<InnerType: BinWrite, ConditionType: BinWrite + Zero + One> BinWrite
    for BffOption<InnerType, ConditionType>
where
    for<'a> <InnerType as BinWrite>::Args<'a>: Default,
    for<'a> <ConditionType as BinWrite>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<R: binrw::io::Write + binrw::io::Seek>(
        &self,
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        ConditionType::write_options(
            &if self.inner.is_some() {
                ConditionType::one()
            } else {
                ConditionType::zero()
            },
            reader,
            endian,
            <_>::default(),
        )?;
        if let Some(data) = &self.inner {
            InnerType::write_options(data, reader, endian, <_>::default())?;
        }
        Ok(())
    }
}

impl<InnerType, ConditionType> From<Option<InnerType>> for BffOption<InnerType, ConditionType>
where
    for<'a> InnerType: BinRead + BinWrite + Serialize + 'a,
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <InnerType as BinWrite>::Args<'a>: Default,

    ConditionType: BinRead + BinWrite + Debug + Copy + PartialEq + One + Zero,
    for<'a> <ConditionType as BinRead>::Args<'a>: Default,
    for<'a> <ConditionType as BinWrite>::Args<'a>: Default,
{
    fn from(inner: Option<InnerType>) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}
