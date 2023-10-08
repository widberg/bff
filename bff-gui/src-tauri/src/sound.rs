use std::path::Path;

use bff::class::sound::Sound;
use bff::names::Name;

use crate::error::BffGuiResult;
use crate::traits::Export;

impl Export for Box<Sound> {
    fn export(&self, export_path: &Path, _name: Name) -> BffGuiResult<String> {
        match **self {
            Sound::SoundV1_291_03_06PC(ref sound) => {
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: sound.body.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut parent_writer = hound::WavWriter::create(export_path, spec).unwrap();
                let mut writer = parent_writer.get_i16_writer(sound.body.data.len() as u32);

                for sample in &sound.body.data {
                    writer.write_sample(*sample);
                }
                writer.flush()?;
                parent_writer.finalize()?;

                Ok(serde_json::to_string_pretty(&sound.body)?)
            }
            Sound::SoundV1_381_67_09PC(ref sound) => {
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: sound.link_header.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                let mut parent_writer = hound::WavWriter::create(export_path, spec).unwrap();
                let mut writer = parent_writer.get_i16_writer(sound.body.data.len() as u32);

                for sample in &sound.body.data {
                    writer.write_sample(*sample);
                }
                writer.flush()?;
                parent_writer.finalize()?;

                Ok(serde_json::to_string_pretty(&sound.body)?)
            }
        }
    }
}
