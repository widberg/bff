use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use quick_xml::writer::Writer;
use serde::Serialize;

use bff::class::mesh::{v1_291_03_06_pc, Mesh};

use crate::error::{GuiError, SimpleError};
use crate::traits::Export;

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

impl Export for Box<Mesh> {
    fn export(&self, export_path: &PathBuf, name: u32) -> Result<String, GuiError> {
        match **self {
            Mesh::MeshV1_291_03_06PC(ref mesh) => {
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
                                    position, ..
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
                                v1_291_03_06_pc::VertexStruct::VertexStructUnknown { .. } => {
                                    (&[0.0; 3], &[0; 3])
                                }
                            })
                            .unzip();
                        let all_positions = positions.iter().flat_map(|pos| pos.to_vec()).collect();
                        let all_normals = normals
                            .iter()
                            .flat_map(|pos| pos.to_vec())
                            .map(|i| (i as f32 - 128.0) / 128.0)
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
                    .flat_map(|tri| tri.indices().iter().rev().map(|i| *i).collect::<Vec<i16>>())
                    .collect();
                let geometries: Vec<ColladaGeometry> = buffers
                    .iter()
                    .enumerate()
                    .map(|(i, buf)| {
                        let geometry_id = format!("{}_{}", name, i);
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
                                        let source_id_array = format!("{}-array", source_id);
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
                                triangles: mesh
                                    .body()
                                    .mesh_buffer()
                                    .vertex_groups()
                                    .iter()
                                    .map(|group| {
                                        let offset_indices = &indices[*group
                                            .index_buffer_offset_in_shorts()
                                            as usize
                                            ..*group.index_buffer_offset_in_shorts() as usize
                                                + *group.face_count() as usize * 3];
                                        ColladaTriangles {
                                            count: offset_indices.len() / 3,
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
                                            p: offset_indices
                                                .iter()
                                                .map(|i| {
                                                    format!("{} {}", i.to_string(), i.to_string())
                                                })
                                                .collect::<Vec<String>>()
                                                .join(" "),
                                        }
                                    })
                                    .collect(),
                            },
                            id: geometry_id,
                            name: format!("{}.{}", name, i),
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

                let mut buffer = Vec::new();
                let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
                writer.write_serializable("COLLADA", &collada)?;
                File::create(&export_path)?.write_all(&buffer)?;
                Ok(serde_json::to_string_pretty(mesh.link_header())?)
            }
            _ => Err(GuiError::Simple(SimpleError(
                "Unimplemented class".to_string(),
            ))),
        }
    }
}
