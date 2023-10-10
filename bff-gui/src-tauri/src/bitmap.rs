use std::io::{BufReader, Cursor};
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use bff::class::bitmap::Bitmap;
use bff::names::Name;

use crate::error::BffGuiResult;
use crate::traits::Export;
use crate::{DataType, PreviewData};

impl Export for Box<Bitmap> {
    fn export(&self, _export_path: &Path, _name: Name) -> BffGuiResult<PreviewData> {
        match **self {
            Bitmap::BitmapV1_381_67_09PC(ref bitmap) => {
                let buf = BufReader::new(Cursor::new(&bitmap.body.data));
                let dds = ddsfile::Dds::read(buf)?;
                let image = image_dds::image_from_dds(&dds, 0)?;
                let mut bytes: Vec<u8> = Vec::new();
                image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
                Ok(PreviewData {
                    is_base64: true,
                    data: general_purpose::STANDARD_NO_PAD.encode(bytes),
                    data_type: DataType::Image,
                })
            }
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
            Bitmap::BitmapV1_06_63_02PC(ref bitmap) => {
                let image: image::ImageBuffer<_, _> = match &bitmap.body.dds {
                    Some(dds_data) => {
                        let buf = BufReader::new(Cursor::new(dds_data));
                        let dds = ddsfile::Dds::read(buf)?;
                        image_dds::image_from_dds(&dds, 0)?
                    }
                    None => {
                        // let buf = BufReader::new(Cursor::new(bitmap.body.tex().unwrap()));
                        match bitmap.body.format {
                            12 => {
                                let inverted_image: Vec<u8> = bitmap
                                    .body
                                    .tex
                                    .as_ref()
                                    .unwrap()
                                    .chunks(3)
                                    .flat_map(|rgb| rgb.iter().rev().copied().collect::<Vec<u8>>())
                                    .collect();
                                image::RgbaImage::from_raw(
                                    bitmap.body.size.0,
                                    bitmap.body.size.1,
                                    inverted_image,
                                )
                            }
                            .unwrap(),
                            _ => {
                                let inverted_image: Vec<u8> = bitmap
                                    .body
                                    .tex
                                    .as_ref()
                                    .unwrap()
                                    .chunks(4)
                                    .flat_map(|rgba| {
                                        let rgb = &rgba[..3];
                                        let mut rev_rgba =
                                            rgb.iter().rev().copied().collect::<Vec<u8>>();
                                        rev_rgba.push(*rgba.last().unwrap());
                                        rev_rgba
                                    })
                                    .collect();
                                image::DynamicImage::ImageRgb8(
                                    image::RgbImage::from_raw(
                                        bitmap.body.size.0,
                                        bitmap.body.size.1,
                                        inverted_image,
                                    )
                                    .unwrap(),
                                )
                                .into_rgba8()
                            }
                        }
                    }
                };
                let mut bytes: Vec<u8> = Vec::new();
                image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
                Ok(PreviewData {
                    is_base64: true,
                    data: general_purpose::STANDARD_NO_PAD.encode(bytes),
                    data_type: DataType::Image,
                })
            }
        }
    }
}
