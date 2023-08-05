// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use bff::bigfile::BigFile;
use bff::class::bitmap::Bitmap;
use bff::class::mesh::{v1_291_03_06_pc, Mesh};
use bff::class::sound::Sound;
use bff::class::user_define::UserDefine;
use bff::class::Class;
use bff::object::Object;
use bff::platforms::Platform;
use bff::traits::TryIntoVersionPlatform;
use bff::BufReader;

use quick_xml::events::{BytesStart, Event};
use quick_xml::writer::Writer;

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

#[derive(Serialize)]
struct Collada {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@xmlns:xsi")]
    xmlnsxsi: String,
    asset: ColladaAsset,
    library_geometries: ColladaLibraryGeometries,
    library_visual_scenes: ColladaLibraryVisualScenes,
    scene: ColladaScene,
}

#[derive(Serialize)]
struct ColladaAsset {
    created: String,
    modified: String,
}

#[derive(Serialize)]
struct ColladaLibraryGeometries {
    geometry: Vec<ColladaGeometry>,
}

#[derive(Serialize)]
struct ColladaGeometry {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    mesh: ColladaMesh,
}

#[derive(Serialize)]
struct ColladaMesh {
    source: Vec<ColladaSource>,
    vertices: ColladaVertices,
    triangles: Vec<ColladaTriangles>,
}

#[derive(Serialize)]
struct ColladaVertices {
    #[serde(rename = "@id")]
    id: String,
    input: ColladaInput,
}

#[derive(Serialize)]
struct ColladaInput {
    #[serde(rename = "@semantic")]
    semantic: String,
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@offset")]
    offset: Option<usize>,
}

#[derive(Serialize)]
struct ColladaTriangles {
    #[serde(rename = "@material")]
    material: Option<String>,
    #[serde(rename = "@count")]
    count: usize,
    input: Vec<ColladaInput>,
    p: String,
}

#[derive(Serialize)]
struct ColladaSource {
    #[serde(rename = "@id")]
    id: String,
    float_array: ColladaFloatArray,
    technique_common: ColladaTechniqueCommon,
}

#[derive(Serialize)]
struct ColladaFloatArray {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@count")]
    count: usize,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Serialize)]
struct ColladaTechniqueCommon {
    accessor: ColladaAccessor,
}

#[derive(Serialize)]
struct ColladaAccessor {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@count")]
    count: usize,
    #[serde(rename = "@stride")]
    stride: usize,
    param: Vec<ColladaParam>,
}

#[derive(Serialize)]
struct ColladaParam {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: String,
}

#[derive(Serialize)]
struct ColladaLibraryVisualScenes {
    visual_scene: Vec<ColladaVisualScene>,
}

#[derive(Serialize)]
struct ColladaVisualScene {
    #[serde(rename = "@id")]
    id: String,
    node: Vec<ColladaNode>,
}

#[derive(Serialize)]
struct ColladaNode {
    instance_geometry: ColladaInstanceGeometry,
}

#[derive(Serialize)]
struct ColladaMatrix {
    #[serde(rename = "@sid")]
    sid: String,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Serialize)]
struct ColladaInstanceGeometry {
    #[serde(rename = "@url")]
    url: String,
}

#[derive(Serialize)]
struct ColladaScene {
    instance_visual_scene: Vec<ColladaInstanceVisualScene>,
}

#[derive(Serialize)]
struct ColladaInstanceVisualScene {
    #[serde(rename = "@url")]
    url: String,
}

struct SimpleMesh {
    positions: Vec<f32>,
    normals: Vec<f32>,
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
                Class::Mesh(mesh) => match *mesh {
                    Mesh::MeshV1_291_03_06PC(mesh) => {
                        let buffers: Vec<SimpleMesh> = mesh
                            .body()
                            .mesh_buffer()
                            .vertex_buffers()
                            .iter()
                            .map(|buf| {
                                let (positions, normals): (Vec<&[f32; 3]>, Vec<&[u8; 3]>) = buf
                                    .vertex_structs()
                                    .iter()
                                    .map(|vstr| match vstr {
                                        v1_291_03_06_pc::VertexStruct::VertexStruct24 {
                                            position,
                                            ..
                                        } => (position, &[0; 3]),
                                        v1_291_03_06_pc::VertexStruct::VertexStruct36 {
                                            position,
                                            normal,
                                            ..
                                        } => (position, normal),
                                        v1_291_03_06_pc::VertexStruct::VertexStruct48 {
                                            position,
                                            normal,
                                            ..
                                        } => (position, normal),
                                        v1_291_03_06_pc::VertexStruct::VertexStruct60 {
                                            position,
                                            normal,
                                            ..
                                        } => (position, normal),
                                        v1_291_03_06_pc::VertexStruct::VertexStructUnknown {
                                            ..
                                        } => (&[0.0; 3], &[0; 3]),
                                    })
                                    .unzip();
                                let all_positions =
                                    positions.iter().flat_map(|pos| pos.to_vec()).collect();
                                let all_normals = normals
                                    .iter()
                                    .flat_map(|pos| pos.to_vec())
                                    .map(|i| (i as f32 - 128.0) / -128.0)
                                    .collect();
                                SimpleMesh {
                                    positions: all_positions,
                                    normals: all_normals,
                                }
                            })
                            .collect();
                        let indices: Vec<i16> = mesh
                            .body()
                            .mesh_buffer()
                            .index_buffers()
                            .iter()
                            .flat_map(|i| i.tris())
                            .flat_map(|tri| tri.indices().to_vec())
                            .collect();
                        let geometries: Vec<ColladaGeometry> = buffers
                            .iter()
                            .enumerate()
                            .map(|(i, buf)| {
                                let geometry_id = format!("{}_{}", object.name(), i);
                                ColladaGeometry {
                                    mesh: ColladaMesh {
                                        source: vec![&buf.positions, &buf.normals]
                                            .iter()
                                            .enumerate()
                                            .map(|(i, p)| {
                                                let source_id = format!(
                                                    "{}-{}",
                                                    geometry_id,
                                                    match i {
                                                        0 => "positions",
                                                        1 => "normals",
                                                        _ => "other",
                                                    }
                                                );
                                                let source_id_array =
                                                    format!("{}-array", source_id);
                                                ColladaSource {
                                                    id: source_id,
                                                    float_array: ColladaFloatArray {
                                                        id: source_id_array.clone(),
                                                        count: p.len(),
                                                        text: p
                                                            .iter()
                                                            .map(|i| i.to_string())
                                                            .collect::<Vec<String>>()
                                                            .join(" "),
                                                    },
                                                    technique_common: ColladaTechniqueCommon {
                                                        accessor: ColladaAccessor {
                                                            count: p.len() / 3,
                                                            stride: 3,
                                                            source: format!("#{}", source_id_array),
                                                            param: vec!["X", "Y", "Z"]
                                                                .iter()
                                                                .map(|axis| ColladaParam {
                                                                    name: axis.to_string(),
                                                                    r#type: "float".to_string(),
                                                                })
                                                                .collect(),
                                                        },
                                                    },
                                                }
                                            })
                                            .collect(),
                                        vertices: ColladaVertices {
                                            id: format!("{}-vertices", geometry_id),
                                            input: ColladaInput {
                                                semantic: "POSITION".to_string(),
                                                source: format!("#{}-positions", geometry_id),
                                                offset: None,
                                            },
                                        },
                                        triangles: vec![ColladaTriangles {
                                            count: indices.len() / 3,
                                            material: None,
                                            input: vec![
                                                ("VERTEX", "vertices"),
                                                ("NORMAL", "normals"),
                                            ]
                                            .iter()
                                            .enumerate()
                                            .map(|(i, p)| ColladaInput {
                                                semantic: p.0.to_string(),
                                                source: format!(
                                                    "#{}-{}",
                                                    geometry_id,
                                                    p.1.to_string()
                                                ),
                                                offset: Some(i),
                                            })
                                            .collect(),
                                            p: indices
                                                .iter()
                                                .map(|i| {
                                                    format!("{} {}", i.to_string(), i.to_string())
                                                })
                                                .collect::<Vec<String>>()
                                                .join(" "),
                                        }],
                                    },
                                    id: geometry_id,
                                    name: format!("{}.{}", object.name(), i),
                                }
                            })
                            .collect();
                        let collada = Collada {
                            xmlns: "http://www.collada.org/2005/11/COLLADASchema".to_string(),
                            version: "1.4.1".to_string(),
                            xmlnsxsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
                            asset: ColladaAsset {
                                created: "1970-01-01T00:00:00".to_string(),
                                modified: "1970-01-01T00:00:00".to_string(),
                            },
                            library_visual_scenes: ColladaLibraryVisualScenes {
                                visual_scene: vec![ColladaVisualScene {
                                    id: "scene".to_string(),
                                    node: (&geometries)
                                        .iter()
                                        .map(|g| ColladaNode {
                                            instance_geometry: ColladaInstanceGeometry {
                                                url: format!("#{}", g.id),
                                            },
                                        })
                                        .collect(),
                                }],
                            },
                            library_geometries: ColladaLibraryGeometries {
                                geometry: geometries,
                            },
                            scene: ColladaScene {
                                instance_visual_scene: vec![ColladaInstanceVisualScene {
                                    url: "#scene".to_string(),
                                }],
                            },
                        };

                        let dae_path = temp_path.join(object.name().to_string() + ".dae");
                        let mut buffer = Vec::new();
                        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
                        // let start = BytesStart::new("COLLADA");
                        // let end = start.to_end();
                        // writer.write_event(Event::Start(start.clone())).unwrap();
                        match writer.write_serializable("COLLADA", &collada) {
                            Err(e) => println!("{}", e),
                            _ => (),
                        }
                        // writer.write_event(Event::End(end)).unwrap();
                        let mut file = File::create(&dae_path).unwrap();
                        match file.write_all(&buffer) {
                            Ok(_) => (
                                format!("{}", serde_yaml::to_string(&object).unwrap()),
                                Some(dae_path.to_str().unwrap().to_string()),
                            ),
                            Err(e) => (
                                format!("{}\n{}", e, serde_yaml::to_string(&object).unwrap()),
                                None,
                            ),
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
