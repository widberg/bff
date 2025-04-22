use std::io::{Read, Seek, Write};

use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceLinkHeader, Vec2f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[bitsize(7)]
#[derive(TryFromBits, Debug, Serialize, Deserialize, ReferencedNames)]
enum SubType {
    ShortCutForest = 0,
    ShortCutField = 1,
    ShortCutShort = 2,
    ShortCutLong = 3,
    FieldRoad = 4,
    GoatPath = 5,      // Bike trail lined with thin posts
    SmallDirtRoad = 6, // Vehicle trail lines with A-frames
    SnowyDirtRoad = 7,
    NormalDirtRoad = 8,     // Two vehicle roads lined with signs and guard rails
    BigDirtRoad = 9,        // Even wider I guess
    SmallCircuitTrack = 10, // Thin track around Redrock Bluffs
    SmallTarmacRoad = 11,
    NormalTarmacRoad = 12,
    BigTarmacRoad = 13,
    River = 14,
    CircuitTrack = 15, // Small tracks near Offshore Shack
    SaltRoad = 16,
    Bridge = 17,
    HighWay = 18,
}

#[bitsize(8)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
struct RoadType {
    sub_type: SubType,
    short_cut: bool,
}

#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames)]
#[serde(transparent)]
struct EncodedPoint(Vec2f);

impl BinRead for EncodedPoint {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let a = i32::read_be(reader)?;
        let b = u8::read_be(reader)?;
        Ok(Self([
            // 2 packed 20-bit signed integers this takes 5 bytes which is 3 less than the 8 it
            // would take to store 2 floats. Sign extend and divide by 4 to get the float value.
            // a           b
            // 01 23 45 67 89
            // \_____/\_____/
            //    x      y
            (a >> 12) as f32 / 4.,
            (((a << 20) >> 12) | b as i32) as f32 / 4.,
        ]))
    }
}

impl BinWrite for EncodedPoint {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let x = (self[0] * 4.).round() as i32;
        let y = (self[1] * 4.).round() as i32;
        let a: i32 = (x << 12) | ((y >> 8) & 0x0FFF);
        let b: u8 = y as u8;

        writer.write_be(&a)?;
        writer.write_be(&b)?;
        Ok(())
    }
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Road {
    r#type: RoadType,
    points: DynArray<EncodedPoint, u16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused5 {
    unused0: u32,
    unused1: u32,
    unused2: u32,
    unused3: u32,
    unused4: u32,
    unused5: u32,
    unused6: u32,
    unused7: u32,
    #[br(count = unused0 & 0xFFFF)]
    unused8s: Vec<u32>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeader))]
pub struct GwRoadBodyV1_381_67_09PC {
    road_count: u32,
    gen_road_min: Vec2f,
    gen_road_max: Vec2f,
    #[br(count = road_count)]
    roads: Vec<Road>,
    unused5_count: u32,
    unused5_min: Vec2f,
    unused5_max: Vec2f,
    #[br(count = unused5_count)]
    unused5s: Vec<Unused5>,
    gen_world_name: Name,
}

pub type GwRoadV1_381_67_09PC = TrivialClass<ResourceLinkHeader, GwRoadBodyV1_381_67_09PC>;

impl Export for GwRoadV1_381_67_09PC {}
impl Import for GwRoadV1_381_67_09PC {}
