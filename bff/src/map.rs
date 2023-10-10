use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use derive_more::Deref;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deref, Deserialize, ReferencedNames)]
#[serde(transparent)]
pub struct BffMap<KeyType: Eq + Hash, ValueType, SizeType = u32> {
    #[deref]
    map: IndexMap<KeyType, ValueType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<KeyType: Eq + Hash + BinRead, ValueType: BinRead, SizeType: BinRead> BinRead
    for BffMap<KeyType, ValueType, SizeType>
where
    for<'a> <KeyType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <ValueType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,
{
    type Args<'a> = ();

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let size = SizeType::read_options(reader, endian, <_>::default())?;

        let mut map = IndexMap::new();

        for _ in 0..size.try_into().unwrap() {
            let key = KeyType::read_options(reader, endian, <_>::default())?;
            let value = ValueType::read_options(reader, endian, <_>::default())?;
            map.insert(key, value);
        }

        Ok(BffMap {
            map,
            _phantom: PhantomData,
        })
    }
}

impl<KeyType: Eq + Hash + BinWrite, ValueType: BinWrite, SizeType: BinWrite> BinWrite
    for BffMap<KeyType, ValueType, SizeType>
where
    for<'a> <KeyType as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <ValueType as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinWrite>::Args<'a>: Default,
    usize: TryInto<SizeType>,
    <usize as TryInto<SizeType>>::Error: Debug,
{
    type Args<'a> = ();

    fn write_options<R: binrw::io::Write + binrw::io::Seek>(
        &self,
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        SizeType::write_options(
            &self.map.len().try_into().unwrap(),
            reader,
            endian,
            <_>::default(),
        )?;

        for (key, value) in self.map.iter() {
            KeyType::write_options(key, reader, endian, <_>::default())?;
            ValueType::write_options(value, reader, endian, <_>::default())?;
        }

        Ok(())
    }
}

impl<KeyType: Eq + Hash, ValueType, SizeType> From<IndexMap<KeyType, ValueType>>
    for BffMap<KeyType, ValueType, SizeType>
{
    fn from(map: IndexMap<KeyType, ValueType>) -> Self {
        Self {
            map,
            _phantom: PhantomData,
        }
    }
}
