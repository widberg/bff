use std::io::{Read, Seek, SeekFrom};

use binrw::{args, binread, parser, BinRead, BinResult};
use xbadpcm::XboxADPCMDecoder;

use crate::bigfile::platforms::Platform;
use crate::names::{names, Name, NameType};

pub struct Sound {
    pub name: Name,
    pub sample_rate: u16,
    pub header: Vec<u8>,
    pub data: [Vec<i16>; 1],
}

#[parser(reader)]
fn sounds_parser(sound_descriptions: &Vec<SoundDescription>) -> BinResult<Vec<Sound>> {
    let mut sounds = Vec::new();

    for sound_description in sound_descriptions {
        reader.seek(SeekFrom::Start(sound_description.header_offset as u64))?;
        let header = Vec::<u8>::read_args(
            reader,
            args! { count: sound_description.header_size as usize },
        )?;
        let data = Vec::<u8>::read_args(
            reader,
            args! { count: sound_description.size as usize - sound_description.header_size as usize },
        )?;

        let mut decoded_data = [Vec::new()];
        let mut decoder = XboxADPCMDecoder::new(1, &mut decoded_data);
        decoder.decode(&data).unwrap();

        sounds.push(Sound {
            name: sound_description.name,
            sample_rate: sound_description.sample_rate,
            header,
            data: decoded_data,
        });
    }

    Ok(sounds)
}

#[derive(BinRead)]
pub struct SoundDescription {
    name: Name,
    sample_rate: u16,
    header_size: u16,
    header_offset: u32,
    size: u32,
}

#[binread]
pub struct SoundBF {
    pub flags: u32,
    #[br(temp)]
    sound_count: u32,
    #[br(temp)]
    _zero: u64,
    #[br(temp, count = sound_count)]
    sound_descriptions: Vec<SoundDescription>,
    #[br(parse_with = sounds_parser, args(&sound_descriptions))]
    pub sounds: Vec<Sound>,
}

impl SoundBF {
    pub fn read_platform<R: Read + Seek>(reader: &mut R, platform: Platform) -> BinResult<Self> {
        names().lock().unwrap().name_type = NameType::Asobo32;
        Self::read_options(reader, platform.into(), ())
    }
}
