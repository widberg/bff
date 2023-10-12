use std::io::{BufReader, Cursor};

use base64::engine::general_purpose;
use base64::Engine as _;
use bff::class::bitmap::Bitmap;
use bff::names::Name;

use crate::error::BffGuiResult;
use crate::traits::Export;
use crate::{DataType, PreviewData};

impl Export for Bitmap {
    fn export(&self) {
        match *self {
            Bitmap::BitmapV1_291_03_06PC(ref bitmap) => {
                let buf = BufReader::new(Cursor::new(&bitmap.body.data));
                let dds = ddsfile::Dds::read(buf)?;
                let image = image_dds::image_from_dds(&dds, 0)?;
                // image.save(export_path)?;
                let mut bytes: Vec<u8> = Vec::new();
                image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
                Ok(PreviewData {
                    is_base64: true,
                    data: general_purpose::STANDARD_NO_PAD.encode(bytes),
                    data_type: DataType::Image,
                })
            }
            _ => {
                todo!()
            }
        }
    }
}
