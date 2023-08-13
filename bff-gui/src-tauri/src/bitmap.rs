use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use bff::class::bitmap::Bitmap;

use crate::{error::GuiError, traits::Export};

impl Export for Box<Bitmap> {
    fn export(&self, export_path: &PathBuf, _name: u32) -> Result<String, GuiError> {
        match **self {
            Bitmap::BitmapV1_291_03_06PC(ref bitmap) => {
                let buf = BufReader::new(Cursor::new(bitmap.body().data()));
                let dds = ddsfile::Dds::read(buf)?;
                let image = image_dds::image_from_dds(&dds, 0)?;
                image.save(export_path)?;
                Ok(serde_yaml::to_string(bitmap.body())?)
            }
            Bitmap::BitmapV1_06_63_02PC(ref bitmap) => {
                let image: image::ImageBuffer<_, _> = match bitmap.body().dds() {
                    Some(dds_data) => {
                        let buf = BufReader::new(Cursor::new(dds_data));
                        let dds = ddsfile::Dds::read(buf)?;
                        image_dds::image_from_dds(&dds, 0)?
                    }
                    None => {
                        // let buf = BufReader::new(Cursor::new(bitmap.body().tex().unwrap()));
                        match bitmap.body().format() {
                            12 => {
                                let inverted_image: Vec<u8> = bitmap
                                    .body()
                                    .tex()
                                    .unwrap()
                                    .chunks(3)
                                    .flat_map(|rgb| {
                                        rgb.iter().rev().map(|i| *i).collect::<Vec<u8>>()
                                    })
                                    .collect();
                                image::RgbaImage::from_raw(
                                    bitmap.body().size().0,
                                    bitmap.body().size().1,
                                    inverted_image,
                                )
                            }
                            .unwrap(),
                            _ => {
                                let inverted_image: Vec<u8> = bitmap
                                    .body()
                                    .tex()
                                    .unwrap()
                                    .chunks(4)
                                    .flat_map(|rgba| {
                                        let rgb = &rgba[..3];
                                        let mut rev_rgba =
                                            rgb.iter().rev().map(|i| *i).collect::<Vec<u8>>();
                                        rev_rgba.push(*rgba.last().unwrap());
                                        rev_rgba
                                    })
                                    .collect();
                                image::DynamicImage::ImageRgb8(
                                    image::RgbImage::from_raw(
                                        bitmap.body().size().0,
                                        bitmap.body().size().1,
                                        inverted_image,
                                    )
                                    .unwrap(),
                                )
                                .into_rgba8()
                            }
                        }
                    }
                };
                image.save(export_path)?;
                Ok(serde_yaml::to_string(bitmap.body())?)
            }
        }
    }
}
