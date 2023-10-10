use std::io::Cursor;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use bff::class::sound::Sound;
use bff::names::Name;

use crate::error::BffGuiResult;
use crate::traits::Export;
use crate::{DataType, PreviewData};

impl Export for Box<Sound> {
    fn export(&self, _export_path: &Path, _name: Name) -> BffGuiResult<PreviewData> {
        match **self {
            Sound::SoundV1_291_03_06PC(ref sound) => {
                let spec = hound::WavSpec {
                    channels: match sound.body.flags.stereo().value() {
                        1 => 2,
                        _ => 1,
                    },
                    sample_rate: sound.body.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut bytes = Vec::new();
                let mut cursor = Cursor::new(&mut bytes); //TODO: use bufwriter
                let mut parent_writer = hound::WavWriter::new(&mut cursor, spec).unwrap();
                let mut sample_writer = parent_writer.get_i16_writer(sound.body.data.len() as u32);

                for sample in &sound.body.data {
                    sample_writer.write_sample(*sample);
                }
                sample_writer.flush()?;
                parent_writer.finalize()?;

                Ok(PreviewData {
                    is_base64: true,
                    data: general_purpose::STANDARD_NO_PAD.encode(bytes),
                    data_type: DataType::Sound,
                })
            }
            Sound::SoundV1_381_67_09PC(ref sound) => {
                let spec = hound::WavSpec {
                    channels: match sound.link_header.flags.stereo().value() {
                        1 => 2,
                        _ => 1,
                    },
                    sample_rate: sound.link_header.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut bytes = Vec::new();
                let mut cursor = Cursor::new(&mut bytes);
                let mut parent_writer = hound::WavWriter::new(&mut cursor, spec).unwrap();
                let mut sample_writer = parent_writer.get_i16_writer(sound.body.data.len() as u32);

                for sample in &sound.body.data {
                    sample_writer.write_sample(*sample);
                }
                sample_writer.flush()?;
                parent_writer.finalize()?;

                Ok(PreviewData {
                    is_base64: true,
                    data: general_purpose::STANDARD_NO_PAD.encode(bytes),
                    data_type: DataType::Sound,
                })
            }
        }
    }
}
