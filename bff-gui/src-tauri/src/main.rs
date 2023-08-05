// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use bff::bigfile::BigFile;
use bff::class::bitmap::Bitmap;
use bff::class::sound::Sound;
use bff::class::user_define::UserDefine;
use bff::class::Class;
use bff::object::Object;
use bff::platforms::Platform;
use bff::traits::TryIntoVersionPlatform;
use bff::BufReader;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PreviewObject {
    name: u32,
    preview_data: String,
    preview_path: Option<String>,
}

#[derive(Serialize)]
struct BigFileData {
    name: String,
    objects: Vec<ObjectData>,
}

#[derive(Serialize)]
struct ObjectData {
    name: u32,
    class_name: u32,
    is_implemented: bool,
}

pub struct InnerAppState {
    bigfile: BigFile,
    platform: Platform,
    preview_objects: Vec<PreviewObject>,
}

impl InnerAppState {
    pub fn bigfile(&self) -> &BigFile {
        &self.bigfile
    }
    pub fn platform(&self) -> &Platform {
        &self.platform
    }
    pub fn preview_objects(&self) -> &Vec<PreviewObject> {
        &self.preview_objects
    }
    pub fn add_preview(&mut self, preview: PreviewObject) {
        self.preview_objects.push(preview);
    }
}

pub struct AppState(pub Mutex<Option<InnerAppState>>);

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![extract_bigfile, parse_object])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//TODO: use thiserr for error propagation
//check tauri docs
#[tauri::command]
fn extract_bigfile(path: &str, state: tauri::State<AppState>) -> Result<BigFileData, String> {
    let bigfile_path = Path::new(path);
    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap(),
        None => Platform::PC,
    };
    let f;
    match File::open(bigfile_path) {
        Ok(x) => f = x,
        Err(_) => return Err("failed to open file".to_string()),
    }
    let mut reader = BufReader::new(f);
    let bigfile;
    match BigFile::read_platform(&mut reader, platform) {
        Ok(x) => bigfile = x,
        Err(_) => return Err("failed to read bigfile".to_string()),
    }

    let objects: Vec<ObjectData> = bigfile
        .blocks()
        .iter()
        .flat_map(|block| block.objects())
        .map(|obj| {
            let class: Result<Class, _> =
                obj.try_into_version_platform(bigfile.header().version().unwrap(), platform);
            return ObjectData {
                name: obj.name(),
                class_name: obj.class_name(),
                is_implemented: match class {
                    Err(_) => false,
                    _ => true,
                },
            };
        })
        .collect();

    let mut state_guard = state.0.lock().unwrap();
    *state_guard = Some(InnerAppState {
        bigfile,
        platform,
        preview_objects: Vec::new(),
    });

    Ok(BigFileData {
        name: bigfile_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        objects,
    })
}

#[tauri::command]
#[allow(unreachable_patterns)]
fn parse_object(
    object_name: u32,
    temp_path: &Path,
    state: tauri::State<AppState>,
) -> PreviewObject {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    let object_names: Vec<u32> = state.preview_objects().iter().map(|obj| obj.name).collect();
    if object_names.contains(&object_name) {
        let preview_object: &PreviewObject = state
            .preview_objects()
            .iter()
            .filter(|obj| obj.name == object_name)
            .next()
            .unwrap();
        return preview_object.clone();
    }
    let bf = state.bigfile();
    let version = bf.header().version().unwrap();
    let platform = state.platform;

    let object: &Object = bf
        .blocks()
        .iter()
        .flat_map(|block| block.objects())
        .filter(|obj| obj.name() == object_name)
        .next()
        .unwrap();

    let new_object = match object.try_into_version_platform(version, platform) {
        Ok(class) => {
            let (data, path) = match class {
                Class::Bitmap(bitmap) => match *bitmap {
                    Bitmap::BitmapV1_291_03_06PC(bitmap_pc) => {
                        let png_path = temp_path.join(object.name().to_string() + ".png");
                        match dds_from_data(bitmap_pc.body().data()) {
                            Ok(dds) => match write_png_from_dds(&dds, &png_path) {
                                Ok(_) => (
                                    format!("{}", serde_yaml::to_string(&object).unwrap()),
                                    Some(png_path.to_str().unwrap().to_string()),
                                ),
                                Err(e) => (e, None),
                            },
                            Err(e) => (format!("{}", e), None),
                        }
                    }
                    _ => (
                        format!(
                            "Error: unimplemented class version\n\n{}",
                            serde_yaml::to_string(&object).unwrap()
                        ),
                        None,
                    ),
                },
                Class::Sound(sound) => match *sound {
                    Sound::SoundV1_291_03_06PC(sound_pc) => {
                        let wav_path = temp_path.join(object.name().to_string() + ".wav");
                        match write_wav_from_data(
                            sound_pc.body().sample_rate(),
                            sound_pc.body().data(),
                            &wav_path,
                        ) {
                            Ok(_) => (
                                format!("{}", serde_yaml::to_string(&object).unwrap()),
                                Some(wav_path.to_str().unwrap().to_string()),
                            ),
                            Err(e) => (e, None),
                        }
                    }
                    _ => (
                        format!(
                            "Error: unimplemented class version\n\n{}",
                            serde_yaml::to_string(&object).unwrap()
                        ),
                        None,
                    ),
                },
                Class::UserDefine(userdefine) => match *userdefine {
                    UserDefine::UserDefineV1_291_03_06PC(userdefine) => {
                        (userdefine.body().data().to_string(), None)
                    }
                    _ => (
                        format!(
                            "Error: unimplemented class version\n\n{}",
                            serde_yaml::to_string(&object).unwrap()
                        ),
                        None,
                    ),
                },
                _ => (
                    format!(
                        "Preview unavailable\n\n{}",
                        serde_yaml::to_string(&object).unwrap()
                    ),
                    None,
                ),
            };
            PreviewObject {
                name: object.name(),
                preview_data: data,
                preview_path: path,
            }
        }
        Err(e) => PreviewObject {
            name: object.name(),
            preview_data: format!("Error: {}\n{}", e, serde_yaml::to_string(&object).unwrap()),
            preview_path: None,
        },
    };
    state.add_preview(new_object.clone());
    new_object
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
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut parent_writer = hound::WavWriter::create(path, spec).unwrap();
    let mut writer = parent_writer.get_i16_writer(data.len() as u32);

    for sample in data {
        writer.write_sample(*sample);
    }
    writer.flush().unwrap();
    parent_writer.finalize().unwrap();
    Ok(())
}
