use std::io::Seek;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binread};
use serde::{Deserialize, Serialize};

use super::{
    DynArray,
    NumeratorFloat,
    Vec,
    Vec2f,
    Vec2i16,
    Vec3f,
    Vec4f,
    Vec4i16,
    calculate_padding,
};
use crate::Endian;
use crate::names::Name;

type Key = f32;

#[binread]
#[derive(Debug, Serialize, Deserialize, ReferencedNames)]
#[br(stream = s)]
pub struct KeyTgtTpl<T>
where
    for<'a> T: BinRead + BinWrite + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    time: Key,
    #[br(temp, try_calc = s.stream_position())]
    begin: u64,
    value: T,
    tangent_in: T,
    tangent_out: T,
    #[br(temp, try_calc = s.stream_position())]
    end: u64,
    #[br(temp, pad_after = calculate_padding((end - begin) as usize, 4))]
    _padding: (),
}

impl<T> BinWrite for KeyTgtTpl<T>
where
    for<'a> T: BinRead + BinWrite + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.time.write_options(writer, endian, <_>::default())?;

        let begin = writer.stream_position()?;

        self.value.write_options(writer, endian, <_>::default())?;
        self.tangent_in
            .write_options(writer, endian, <_>::default())?;
        self.tangent_out
            .write_options(writer, endian, <_>::default())?;

        let end = writer.stream_position()?;

        vec![0xffu8; calculate_padding((end - begin) as usize, 4)].write_options(
            writer,
            endian,
            <_>::default(),
        )?;
        Ok(())
    }
}

#[binread]
#[derive(Debug, Serialize, Deserialize, ReferencedNames)]
#[br(stream = s)]
pub struct KeyLinearTpl<T>
where
    for<'a> T: BinRead + BinWrite + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    time: Key,
    #[br(temp, try_calc = s.stream_position())]
    begin: u64,
    value: T,
    #[br(temp, try_calc = s.stream_position())]
    end: u64,
    #[br(temp, pad_after = calculate_padding((end - begin) as usize, 4))]
    _padding: (),
}

// TODO: This is a hack to get around the fact that BinWrite doesn't support try_calc + ignore so
// we can't get the stream position while deriving BinWrite.
impl<T> BinWrite for KeyLinearTpl<T>
where
    for<'a> T: BinRead + BinWrite + Serialize + 'a,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.time.write_options(writer, endian, <_>::default())?;

        let begin = writer.stream_position()?;

        self.value.write_options(writer, endian, <_>::default())?;

        let end = writer.stream_position()?;

        vec![0xffu8; calculate_padding((end - begin) as usize, 4)].write_options(
            writer,
            endian,
            <_>::default(),
        )?;
        Ok(())
    }
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u16)]
pub enum KeyframerInterpolationType {
    Smooth = 1,
    Linear = 2,
    Square = 3,
    Unknown4 = 4,   // scroll_keyframer in MaterialAnim uses this
    Unknown8 = 8,   // scroll_keyframer in MaterialAnim uses this
    Unknown17 = 17, // unknown1 in Rtc's RtcAnimationNode uses this
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
pub struct KeyframerTpl<TKey>
where
    for<'a> TKey: BinRead + BinWrite + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
    for<'a> TKey: BinWrite<Args<'a> = ()>,
{
    interpolation_type: KeyframerInterpolationType,
    keyframes: DynArray<TKey>,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
pub struct KeyframerNoFlagsTpl<TKey>
where
    for<'a> TKey: BinRead + BinWrite + Serialize + 'a,
    for<'a> <TKey as BinRead>::Args<'a>: Clone + Default,
    for<'a> TKey: BinWrite<Args<'a> = ()>,
{
    keyframes: DynArray<TKey>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct Message {
    message_class: u32,
    reciever_name: Name,
    c: u32,
    parameter: f32,
    message_name: Name,
}

pub type Vec3Comp = Vec<3, NumeratorFloat<i16, 4096>>;
pub type QuatComp = Vec<4, NumeratorFloat<i16, 2000>>;

pub type KeyFlag = KeyLinearTpl<u32>;
pub type KeyHdl = KeyLinearTpl<Name>;
pub type KeyMessage = KeyLinearTpl<DynArray<Message>>;
pub type KeyFloat = KeyTgtTpl<f32>;
pub type KeyFloatComp = KeyTgtTpl<i16>;
pub type KeyFloatLinear = KeyLinearTpl<f32>;
pub type KeyFloatLinearComp = KeyLinearTpl<i16>;
pub type KeyU32Linear = KeyLinearTpl<u32>;
pub type KeyVec2f = KeyTgtTpl<Vec2f>;
pub type KeyVec2fComp = KeyTgtTpl<Vec2i16>;
pub type KeyVec2fLinear = KeyLinearTpl<Vec2f>;
pub type KeyVec2fLinearComp = KeyLinearTpl<Vec2i16>;
pub type KeyVec3f = KeyTgtTpl<Vec3f>;
pub type KeyVec3fComp = KeyTgtTpl<Vec3Comp>;
pub type KeyVec3fLinear = KeyLinearTpl<Vec3f>;
pub type KeyVec3fLinearComp = KeyLinearTpl<Vec3Comp>;
pub type KeyVec4f = KeyTgtTpl<Vec4f>;
pub type KeyVec4fComp = KeyTgtTpl<Vec4i16>;
pub type KeyVec4fLinear = KeyLinearTpl<Vec4f>;
pub type KeyVec4fLinearComp = KeyLinearTpl<Vec4i16>;
pub type KeyRot = KeyLinearTpl<QuatComp>;
pub type KeyBezierRot = KeyTgtTpl<Vec3f>;

pub type KeyframerFlag = KeyframerNoFlagsTpl<KeyFlag>;
pub type KeyframerHdl = KeyframerNoFlagsTpl<KeyHdl>;
pub type KeyframerMessage = KeyframerNoFlagsTpl<KeyMessage>;
pub type KeyframerFloat = KeyframerTpl<KeyFloat>;
pub type KeyframerFloatComp = KeyframerTpl<KeyFloatComp>;
pub type KeyframerFloatLinear = KeyframerTpl<KeyFloatLinear>;
pub type KeyframerFloatLinearComp = KeyframerTpl<KeyFloatLinearComp>;
pub type KeyframerU32Linear = KeyframerTpl<KeyU32Linear>;
pub type KeyframerVec2f = KeyframerTpl<KeyVec2f>;
pub type KeyframerVec2fComp = KeyframerTpl<KeyVec2fComp>;
pub type KeyframerVec2fLinear = KeyframerTpl<KeyVec2fLinear>;
pub type KeyframerVec2fLinearComp = KeyframerTpl<KeyVec2fLinearComp>;
pub type KeyframerVec3f = KeyframerTpl<KeyVec3f>;
pub type KeyframerVec3fComp = KeyframerTpl<KeyVec3fComp>;
pub type KeyframerVec3fLinear = KeyframerTpl<KeyVec3fLinear>;
pub type KeyframerVec3fLinearComp = KeyframerTpl<KeyVec3fLinearComp>;
pub type KeyframerVec4f = KeyframerTpl<KeyVec4f>;
pub type KeyframerVec4fComp = KeyframerTpl<KeyVec4fComp>;
pub type KeyframerVec4fLinear = KeyframerTpl<KeyVec4fLinear>;
pub type KeyframerVec4fLinearComp = KeyframerTpl<KeyVec4fLinearComp>;
pub type KeyframerRot = KeyframerNoFlagsTpl<KeyRot>;
pub type KeyframerBezierRot = KeyframerNoFlagsTpl<KeyBezierRot>;
