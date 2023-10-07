// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use bff::bigfile::BigFile;
// use bff::class::material::Material;
use bff::class::user_define::UserDefine;
use bff::class::Class;
use bff::object::Object;
use bff::platforms::Platform;
use bff::traits::TryIntoVersionPlatform;

use error::{GuiError, SimpleError};
use serde::Serialize;
use traits::Export;

mod bitmap;
mod error;
mod mesh;
mod sound;
mod traits;

#[derive(Debug, Serialize, Clone)]
pub struct PreviewObject {
    name: u32,
    preview_data: Option<String>,
    preview_path: Option<PathBuf>,
    error: Option<String>,
}

impl std::fmt::Display for PreviewObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.preview_data {
                Some(ref d) => d,
                None => "None",
            }
        )
    }
}

#[derive(Serialize)]
struct BigFileData {
    name: String,
    objects: Vec<ObjectData>,
}

#[derive(Serialize)]
struct ObjectData {
    name: u32,
    real_class_name: Option<String>,
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
        .invoke_handler(tauri::generate_handler![
            extract_bigfile,
            parse_object,
            export_all_objects,
            export_one_object,
            export_preview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn extract_bigfile(path: &str, state: tauri::State<AppState>) -> Result<BigFileData, GuiError> {
    let bigfile_path = Path::new(path);
    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap(),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    let bigfile = BigFile::read_platform(&mut reader, platform)?;

    let objects: Vec<ObjectData> = bigfile
        .blocks()
        .iter()
        .flat_map(|block| block.objects())
        .map(|obj| {
            // let class: Result<Class, _> =
            //     obj.try_into_version_platform(bigfile.header().version().unwrap(), platform);
            let real_name = obj.real_class_name();
            return ObjectData {
                name: obj.name(),
                real_class_name: match real_name {
                    Ok(n) => Some(n.to_string()),
                    Err(_) => None,
                },
                is_implemented: match real_name {
                    Err(_) => false,
                    Ok(_) => true,
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

    let (data, path, err) = match object.try_into_version_platform(version, platform) {
        Ok(class) => {
            let (res, export_path) = match class {
                Class::Bitmap(ref bitmap) => {
                    let new_path = temp_path.join(format!("{}.png", object.name()));
                    (bitmap.export(&new_path, object_name), Some(new_path))
                }
                Class::Sound(ref sound) => {
                    let new_path = temp_path.join(format!("{}.wav", object.name()));
                    (sound.export(&new_path, object_name), Some(new_path))
                }
                Class::Mesh(ref mesh) => {
                    let new_path = temp_path.join(format!("{}.dae", object.name()));
                    (mesh.export(&new_path, object_name), Some(new_path))
                }
                Class::UserDefine(ref userdefine) => match **userdefine {
                    UserDefine::UserDefineV1_291_03_06PC(ref userdefine) => {
                        (Ok(userdefine.body().data().to_string()), None)
                    }
                },
                // Class::Material(material) => match *material {
                //     Material::MaterialV1_291_03_06PC(material) => {}
                //     _ => (),
                // },
                _ => (Ok(serde_json::to_string_pretty(&class).unwrap()), None),
            };
            match res {
                Ok(d) => (Some(d), export_path, None),
                Err(e) => (
                    Some(serde_json::to_string_pretty(&class).unwrap()),
                    None,
                    Some(serde_json::to_string_pretty(&e).unwrap()),
                ),
            }
        }
        Err(e) => (
            Some(serde_json::to_string_pretty(&object).unwrap()),
            None,
            Some(serde_json::to_string_pretty(&GuiError::Bff(e)).unwrap()),
        ),
    };
    let new_object = PreviewObject {
        name: object_name,
        preview_data: data,
        preview_path: path,
        error: err,
    };
    state.add_preview(new_object.clone());
    new_object
}

#[tauri::command]
fn export_all_objects(path: &Path, state: tauri::State<AppState>) -> Result<(), GuiError> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    for object in state.bigfile().blocks().iter().flat_map(|b| b.objects()) {
        let class_res: bff::BffResult<Class> = object.try_into_version_platform(
            state.bigfile().header().version().unwrap(),
            *state.platform(),
        );
        match class_res {
            Ok(class) => write_class(&path.join(format!("{}.json", object.name())), &class)?,
            Err(_) => println!("skipped {}", object.name()),
        }
    }
    Ok(())
}

#[tauri::command]
fn export_one_object(
    path: &Path,
    name: u32,
    state: tauri::State<AppState>,
) -> Result<(), GuiError> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    let object: &Object = state
        .bigfile()
        .blocks()
        .iter()
        .flat_map(|block| block.objects())
        .filter(|obj| obj.name() == name)
        .next()
        .ok_or(SimpleError("failed to find object in bigfile".to_string()))?;
    let class_res: bff::BffResult<Class> = object.try_into_version_platform(
        state.bigfile().header().version().unwrap(),
        *state.platform(),
    );
    match class_res {
        Ok(class) => write_class(&path.join(format!("{}.json", object.name())), &class)?,
        Err(_) => println!("skipped {}", object.name()),
    }
    Ok(())
}

fn write_class(path: &PathBuf, class: &Class) -> Result<(), GuiError> {
    let mut file = File::create(path)?;
    file.write(serde_json::to_string_pretty(&class)?.as_bytes())?;
    Ok(())
}

#[tauri::command]
fn export_preview(path: &Path, name: u32, state: tauri::State<AppState>) -> Result<(), GuiError> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    let preview_object: &PreviewObject = state
        .preview_objects()
        .iter()
        .filter(|obj| obj.name == name)
        .next()
        .unwrap();
    std::fs::copy(preview_object.preview_path.as_ref().unwrap(), path)?;
    Ok(())
}
