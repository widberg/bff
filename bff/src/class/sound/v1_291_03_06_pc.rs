use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Cursor;

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

use super::generic::SoundFlags;
use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::ResourceObjectLinkHeaderV1_06_63_02PC;
use crate::traits::{Artifact, Export, Import};

#[derive(..BffStruct)]
#[br(import(link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct SoundBodyV1_291_03_06PC {
    #[serde(skip)]
    sample_rate: u32,
    #[serde(skip)]
    data_size: u32,
    flags: SoundFlags,
    #[br(count = data_size / 2)]
    #[serde(skip)]
    data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, SoundBodyV1_291_03_06PC>;

impl Export for SoundV1_291_03_06PC {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        let channels = if self.body.flags.stereo() {
            2_u16
        } else {
            1_u16
        };
        let spec = WavSpec {
            channels,
            sample_rate: self.body.sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let mut wav = Cursor::new(Vec::new());
        {
            let mut writer =
                WavWriter::new(&mut wav, spec).map_err(|_| Error::UnimplementedImportExport)?;
            for sample in &self.body.data {
                writer
                    .write_sample(*sample)
                    .map_err(|_| Error::UnimplementedImportExport)?;
            }
            writer
                .finalize()
                .map_err(|_| Error::UnimplementedImportExport)?;
        }

        Ok(HashMap::from([(
            OsString::from("data"),
            Artifact::Wav(wav.into_inner()),
        )]))
    }
}

impl Import for SoundV1_291_03_06PC {
    fn import(&mut self, artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        let data_name = OsString::from("data");
        let Artifact::Wav(data) = artifacts.get(&data_name).ok_or(Error::ImportBadArtifact)?
        else {
            return Err(Error::ImportBadArtifact);
        };

        let mut reader =
            WavReader::new(Cursor::new(data)).map_err(|_| Error::ImportBadArtifact)?;
        let spec = reader.spec();
        if spec.sample_format != SampleFormat::Int
            || spec.bits_per_sample != 16
            || !(spec.channels == 1 || spec.channels == 2)
        {
            return Err(Error::UnimplementedImportExport);
        }

        let samples: Vec<i16> = reader
            .samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Error::ImportBadArtifact)?;
        let data_size = samples
            .len()
            .checked_mul(2)
            .and_then(|size| u32::try_from(size).ok())
            .ok_or(Error::UnimplementedImportExport)?;

        self.body.sample_rate = spec.sample_rate;
        self.body.data_size = data_size;
        self.body
            .flags
            .set_stereo(spec.channels == 2);
        self.body.data = samples;
        Ok(())
    }
}
