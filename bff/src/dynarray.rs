use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binread, BinRead, BinWrite};
use derive_more::Deref;
use serde::{Deserialize, Serialize};

#[binread]
#[derive(Debug, Serialize, Deref, Deserialize)]
#[serde(transparent)]
#[br(import_raw(inner: <InnerType as BinRead>::Args<'_>))]
pub struct DynArray<InnerType: BinRead + 'static, SizeType: BinRead = u32>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,
{
    #[br(temp)]
    #[bw(ignore)]
    count: SizeType,
    #[deref]
    #[br(args { count: count.try_into().unwrap(), inner: inner.clone() })]
    data: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<InnerType: BinWrite + BinRead + 'static, SizeType: BinWrite + BinRead> BinWrite
    for DynArray<InnerType, SizeType>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,
    for<'a> <InnerType as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinWrite>::Args<'a>: Default,
    usize: TryInto<SizeType>,
    <usize as TryInto<SizeType>>::Error: Debug,
{
    type Args<'a> = <InnerType as BinWrite>::Args<'a>;

    fn write_options<R: binrw::io::Write + binrw::io::Seek>(
        &self,
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        SizeType::write_options(
            &self.data.len().try_into().unwrap(),
            reader,
            endian,
            <_>::default(),
        )?;
        Vec::write_options(&self.data, reader, endian, <_>::default())?;
        Ok(())
    }
}

impl<InnerType: BinRead, SizeType: BinRead> From<DynArray<InnerType, SizeType>> for Vec<InnerType>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,
{
    fn from(dynarray: DynArray<InnerType, SizeType>) -> Self {
        dynarray.data
    }
}

impl<InnerType: BinRead, SizeType: BinRead> From<Vec<InnerType>> for DynArray<InnerType, SizeType>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,
{
    fn from(vec: Vec<InnerType>) -> Self {
        Self {
            data: vec,
            _phantom: PhantomData,
        }
    }
}
