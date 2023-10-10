// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use base64::{engine::general_purpose, Engine as _};
use bff::bigfile::resource::Resource;
use bff::bigfile::BigFile;
use bff::class::user_define::UserDefine;
use bff::class::Class;
use bff::names::Name;
use bff::platforms::Platform;
use bff::traits::TryIntoVersionPlatform;
use error::{BffGuiResult, InvalidPreviewError, InvalidResourceError};
use serde::Serialize;
use serde_repr::Serialize_repr;
use traits::Export;

use crate::error::Error;

mod bitmap;
mod error;
mod mesh;
mod sound;
mod traits;

#[derive(Debug, Serialize, Clone)]
pub struct ResourcePreview {
    name: Name,
    preview_json: String,
    preview_data: Option<PreviewData>,
}

impl std::fmt::Display for ResourcePreview {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.preview_json)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PreviewData {
    is_base64: bool,
    data: String,
    data_type: DataType,
}

#[derive(Debug, Serialize_repr, Clone, Copy)]
#[repr(u8)]
enum DataType {
    Image = 0,
    Sound = 1,
    Mesh = 2,
    Text = 3,
}

#[derive(Serialize)]
struct BigFileData {
    filename: String,
    resource_infos: Vec<ResourceInfo>,
}

#[derive(Serialize)]
struct ResourceInfo {
    name: Name,
    class_name: String,
}

pub struct InnerAppState {
    pub bigfile: BigFile,
    pub platform: Platform,
    pub resource_previews: HashMap<Name, ResourcePreview>,
}

impl InnerAppState {
    pub fn add_preview(&mut self, preview: ResourcePreview) {
        self.resource_previews.insert(preview.name, preview);
    }
}

pub struct AppState(pub Mutex<Option<InnerAppState>>);

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            extract_bigfile,
            parse_resource,
            export_all_json,
            export_one_json,
            export_preview,
            get_extensions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn extract_bigfile(path: &str, state: tauri::State<AppState>) -> Result<BigFileData, Error> {
    let bigfile_path = Path::new(path);
    let platform = match bigfile_path.extension() {
        Some(extension) => extension.try_into().unwrap(),
        None => Platform::PC,
    };
    let f = File::open(bigfile_path)?;
    let mut reader = BufReader::new(f);
    let bigfile = BigFile::read_platform(&mut reader, platform)?;

    let resources: Vec<ResourceInfo> = bigfile
        .objects
        .values()
        .map(|res| ResourceInfo {
            name: res.name,
            class_name: res.class_name.to_string(),
        })
        .collect();

    let mut state_guard = state.0.lock().unwrap();
    *state_guard = Some(InnerAppState {
        bigfile,
        platform,
        resource_previews: HashMap::new(),
    });

    Ok(BigFileData {
        filename: bigfile_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        resource_infos: resources,
    })
}

#[tauri::command]
fn parse_resource(
    resource_name: Name,
    temp_path: &Path,
    state: tauri::State<AppState>,
) -> BffGuiResult<ResourcePreview> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    if let Some(resource_preview) = state.resource_previews.get(&resource_name) {
        return Ok(resource_preview.clone());
    }
    let bf = &state.bigfile;
    let version = &bf.manifest.version;
    let platform = state.platform;

    let resource: &Resource = bf.objects.get(&resource_name).unwrap();

    let class = resource.try_into_version_platform(version.clone(), platform)?;
    let data = match class {
        Class::Bitmap(ref bitmap) => {
            let new_path = temp_path.join(format!("{}.png", resource.name));
            Some(bitmap.export(&new_path, resource_name)?)
        }
        Class::Sound(ref sound) => {
            let new_path = temp_path.join(format!("{}.wav", resource.name));
            Some(sound.export(&new_path, resource_name)?)
        }
        Class::Mesh(ref mesh) => {
            let new_path = temp_path.join(format!("{}.dae", resource.name));
            Some(mesh.export(&new_path, resource_name)?)
        }
        Class::UserDefine(ref userdefine) => match **userdefine {
            UserDefine::UserDefineV1_291_03_06PC(ref userdefine) => Some(PreviewData {
                is_base64: false,
                data: userdefine.body.data.to_string(),
                data_type: DataType::Text,
            }),
        },
        // Class::Material(material) => match *material {
        //     Material::MaterialV1_291_03_06PC(material) => {}
        //     _ => (),
        // },
        _ => None,
    };
    let json = serde_json::to_string_pretty(&class)?;

    let new_object = ResourcePreview {
        name: resource_name,
        preview_json: json,
        preview_data: data,
    };
    state.add_preview(new_object.clone());
    Ok(new_object)
}

#[tauri::command]
fn export_all_json(path: &Path, state: tauri::State<AppState>) -> BffGuiResult<()> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    for resource in state.bigfile.objects.values() {
        let class_res: bff::BffResult<Class> = resource
            .try_into_version_platform(state.bigfile.manifest.version.clone(), state.platform);
        match class_res {
            Ok(class) => write_class(&path.join(format!("{}.json", resource.name)), &class)?,
            Err(_) => println!("skipped {}", resource.name),
        }
    }
    Ok(())
}

#[tauri::command]
fn export_one_json(path: &Path, name: Name, state: tauri::State<AppState>) -> BffGuiResult<()> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    let resource: &Resource = state
        .bigfile
        .objects
        .get(&name)
        .ok_or(InvalidResourceError {
            resource_name: name,
        })?;
    let class_res: bff::BffResult<Class> =
        resource.try_into_version_platform(state.bigfile.manifest.version.clone(), state.platform);
    match class_res {
        Ok(class) => write_class(&path.join(format!("{}.json", resource.name)), &class)?,
        Err(_) => println!("skipped {}", resource.name),
    }
    Ok(())
}

fn write_class(path: &PathBuf, class: &Class) -> BffGuiResult<()> {
    File::create(path)?.write_all(serde_json::to_string_pretty(&class)?.as_bytes())?;
    Ok(())
}

#[tauri::command]
fn export_preview(path: &Path, name: Name, state: tauri::State<AppState>) -> BffGuiResult<()> {
    let mut state_guard = state.0.lock().unwrap();
    let state = state_guard.as_mut().unwrap();
    let resource_preview: &ResourcePreview = state
        .resource_previews
        .get(&name)
        .ok_or(InvalidPreviewError::new(name))?;
    let preview_data = resource_preview
        .preview_data
        .as_ref()
        .ok_or(InvalidPreviewError::new(name))?;
    let binding = general_purpose::STANDARD_NO_PAD.decode(&preview_data.data)?;
    let written_data = match preview_data.is_base64 {
        true => &binding,
        false => preview_data.data.as_bytes(),
    };
    File::create(path)?.write(written_data)?;
    Ok(())
}

#[tauri::command]
fn get_extensions() -> Vec<String> {
    bff::platforms::extensions()
        .into_iter()
        .map(|s| s.to_str().unwrap().into())
        .collect()
}
