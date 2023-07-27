// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Instant;

use bff::bigfile::BigFile;
use bff::class::bitmap::Bitmap;
use bff::class::sound::Sound;
use bff::class::Class;
use bff::object::Object;
use bff::platforms::Platform;
use bff::traits::TryIntoVersionPlatform;
use bff::versions::Version;
use bff::BufReader;

use serde::Serialize;

#[derive(Serialize)]
struct ClassData {
    preview_path: Option<String>,
    preview_text: String,
    name: u32,
}

#[derive(Serialize)]
struct BigFileData {
    name: String,
    platform: String,
    version: String,
    objects: Vec<Object>,
}

//TODO: use thiserr for error propagation
//check tauri docs
#[tauri::command]
fn extract_bigfile(path: &str) -> Result<BigFileData, String> {
    let bigfile_path = Path::new(path);
    let ext;
    let platform = match bigfile_path.extension() {
        Some(extension) => {
            ext = extension;
            extension.try_into().unwrap()
        }
        None => {
            ext = OsStr::new("DPC");
            Platform::PC
        }
    };
    let f;
    match File::open(bigfile_path) {
        Ok(x) => f = x,
        Err(_) => return Err("failed to open file".to_string()),
    }
    let now = Instant::now();
    let mut reader = BufReader::new(f);
    let bigfile;
    match BigFile::read_platform(&mut reader, platform) {
        Ok(x) => bigfile = x,
        Err(_) => return Err("failed to read bigfile".to_string()),
    }
    let objects: Vec<Object> = bigfile
        .blocks()
        .iter()
        .flat_map(|block| (*block.objects()).clone())
        // .map(|obj| serde_json::to_string(obj).unwrap())
        .collect();

    let elapsed = now.elapsed();
    println!("Time to parse: {:?}", elapsed);
    Ok(BigFileData {
        name: bigfile_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        platform: ext.to_str().unwrap().to_string(),
        version: Into::<&'static str>::into(bigfile.header().version().unwrap()).to_string(),
        objects,
    })
}

#[tauri::command]
fn parse_object(
    object: Object,
    version_str: &str,
    platform_str: &str,
    temp_path: &Path,
) -> ClassData {
    let version = match version_str.try_into() {
        Ok(x) => x,
        Err(_) => Version::V1_06_63_02,
    };
    let platform = match OsStr::new(platform_str).try_into() {
        Ok(x) => x,
        Err(_) => Platform::PC,
    };
    // let mut object = Object {
    //     name: object_data.name,
    //     class_name: object_data.class_name,
    // };
    // let preview_path = temp_path.join(object.name().to_string());
    match (&object).try_into_version_platform(version, platform) {
        Ok(class) => ClassData {
            name: object.name(),
            preview_text: format!("{}", serde_json::to_string_pretty(&class).unwrap()),
            preview_path: match class {
                Class::Bitmap(bitmap) => match *bitmap {
                    Bitmap::BitmapV1_291_03_06PC(bitmap_pc) => {
                        // let dds_path = temp_path.join(object.name().to_string() + ".dds");
                        let png_path = temp_path.join(object.name().to_string() + ".png");
                        match dds_from_data(bitmap_pc.body().data()) {
                            Ok(dds) => match write_png_from_dds(&dds, &png_path) {
                                Ok(_) => Some(png_path.to_str().unwrap().to_string()),
                                Err(e) => {
                                    println!("{}", e);
                                    None
                                }
                            },
                            Err(_) => {
                                println!("failed to read dds from data");
                                None
                            }
                        }
                    }
                    _ => None,
                },
                Class::Sound(sound) => match *sound {
                    Sound::SoundV1_291_03_06PC(sound_pc) => {
                        let wav_path = temp_path.join(object.name().to_string() + ".wav");
                        match write_wav_from_data(
                            sound_pc.body().sample_rate(),
                            sound_pc.body().data(),
                            &wav_path,
                        ) {
                            Ok(_) => Some(wav_path.to_str().unwrap().to_string()),
                            Err(e) => {
                                println!("{}", e);
                                None
                            }
                        }
                    }
                },
                _ => None,
            },
        },
        Err(e) => ClassData {
            name: object.name(),
            preview_path: None,
            preview_text: format!(
                "Error: {}\n{}",
                e,
                serde_json::to_string_pretty(&object).unwrap()
            ),
        },
    }
}

//TODO: add trait Exportable for bitmaps, sounds, etc.
fn dds_from_data(data: &Vec<u8>) -> Result<ddsfile::Dds, ddsfile::Error> {
    let cursor = Cursor::new(data);
    ddsfile::Dds::read(cursor)
}

fn write_png_from_dds(dds: &ddsfile::Dds, path: &PathBuf) -> Result<(), String> {
    match image_dds::image_from_dds(dds, 0) {
        Ok(x) => match x.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    }
}

fn write_wav_from_data(sample_rate: u32, data: &Vec<i16>, path: &PathBuf) -> Result<(), String> {
    // let cursor = Cursor::new(data);
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut parent_writer = hound::WavWriter::create(path, spec).unwrap();
    let mut writer = parent_writer.get_i16_writer(data.len() as u32);

    // let mut data_cursor = Cursor::new(data);

    for sample in data {
        writer.write_sample(*sample);
    }
    writer.flush().unwrap();
    parent_writer.finalize().unwrap();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extract_bigfile, parse_object])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
