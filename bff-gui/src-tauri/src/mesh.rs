use std::io::Cursor;
use std::str::from_utf8;

use bff::class::mesh::{v1_291_03_06_pc, Mesh};
use bff::names::Name;
use bff::traits::NamedClass;
use quick_xml::writer::Writer;
use serde::Serialize;

use crate::error::{BffGuiResult, UnimplementedExporterError};
use crate::traits::Export;
use crate::{DataType, PreviewData};

structstruck::strike! {
    #[strikethrough[derive(Serialize)]]
    struct Collada {
        #[serde(rename = "@xmlns")]
        xmlns: String,
        #[serde(rename = "@version")]
        version: String,
        #[serde(rename = "@xmlns:xsi")]
        xmlnsxsi: String,
        asset: struct ColladaAsset {
            created: String,
            modified: String,
        },
        library_geometries: struct ColladaLibraryGeometries {
            geometry: Vec<struct ColladaGeometry {
                #[serde(rename = "@id")]
                id: String,
                #[serde(rename = "@name")]
                name: String,
                mesh: struct ColladaMesh {
                    source: Vec<struct ColladaSource {
                        #[serde(rename = "@id")]
                        id: String,
                        float_array: struct ColladaFloatArray {
                            #[serde(rename = "@id")]
                            id: String,
                            #[serde(rename = "@count")]
                            count: usize,
                            #[serde(rename = "$text")]
                            text: String,
                        },
                        technique_common: struct ColladaTechniqueCommon {
                            accessor: struct ColladaAccessor {
                                #[serde(rename = "@source")]
                                source: String,
                                #[serde(rename = "@count")]
                                count: usize,
                                #[serde(rename = "@stride")]
                                stride: usize,
                                param: Vec<struct ColladaParam {
                                    #[serde(rename = "@name")]
                                    name: String,
                                    #[serde(rename = "@type")]
                                    r#type: String,
                                }>,
                            },
                        },
                    }>,
                    vertices: struct ColladaVertices {
                        #[serde(rename = "@id")]
                        id: String,
                        input: ColladaInput,
                    }
                    ,
                    triangles: Vec<
                    struct ColladaTriangles {
                        #[serde(rename = "@material")]
                        material: Option<String>,
                        #[serde(rename = "@count")]
                        count: usize,
                        input: Vec<ColladaInput>,
                        p: String,
                    }>,
                },
            }>,
        },
        library_visual_scenes: struct ColladaLibraryVisualScenes {
            visual_scene: Vec<struct ColladaVisualScene {
                #[serde(rename = "@id")]
                id: String,
                node: Vec<struct ColladaNode {
                    instance_geometry: struct ColladaInstanceGeometry {
                        #[serde(rename = "@url")]
                        url: String,
                    },
                }>,
            }>,
        },
        scene: struct ColladaScene {
            instance_visual_scene: Vec<struct ColladaInstanceVisualScene {
                #[serde(rename = "@url")]
                url: String,
            }>,
        },
    }
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
struct ColladaMatrix {
    #[serde(rename = "@sid")]
    sid: String,
    #[serde(rename = "$text")]
    text: String,
}

struct SimpleMesh {
    positions: Vec<f32>,
    normals: Vec<f32>,
}

impl Export for Box<Mesh> {
    fn export(&self, name: Name) -> BffGuiResult<PreviewData> {
        match **self {
            Mesh::MeshV1_291_03_06PC(ref mesh) => {
                let buffers: Vec<SimpleMesh> = mesh
                    .body
                    .mesh_buffer
                    .vertex_buffers
                    .iter()
                    .map(|buf| {
                        let (positions, normals): (Vec<&[f32; 3]>, Vec<&[u8; 3]>) = buf
                            .vertex_structs
                            .iter()
                            .map(|vstr| match vstr {
                                v1_291_03_06_pc::VertexStruct::VertexStruct24 {
                                    position, ..
                                } => (position, &[0; 3]),
                                v1_291_03_06_pc::VertexStruct::VertexStruct36 {
                                    position,
                                    normal,
                                    ..
                                }
                                | v1_291_03_06_pc::VertexStruct::VertexStruct48 {
                                    position,
                                    normal,
                                    ..
                                }
                                | v1_291_03_06_pc::VertexStruct::VertexStruct60 {
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
                    .body
                    .mesh_buffer
                    .index_buffers
                    .iter()
                    .flat_map(|i| &i.tris)
                    .flat_map(|tri| tri.indices.iter().rev().copied().collect::<Vec<i16>>())
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
                                    .body
                                    .mesh_buffer
                                    .vertex_groups
                                    .iter()
                                    .map(|group| {
                                        let offset_indices =
                                            &indices[group.index_buffer_offset_in_shorts as usize
                                                ..group.index_buffer_offset_in_shorts as usize
                                                    + group.face_count as usize * 3];
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
                                                source: format!("#{}-{}", geometry_id, p.1),
                                                offset: Some(i),
                                            })
                                            .collect(),
                                            p: offset_indices
                                                .iter()
                                                .map(|i| format!("{} {}", i, i))
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
                            node: geometries
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

                let mut bytes = Vec::new();
                let mut writer = Writer::new_with_indent(Cursor::new(&mut bytes), b' ', 2);
                writer.write_serializable("COLLADA", &collada)?;
                Ok(PreviewData {
                    is_base64: true,
                    data: from_utf8(&bytes).unwrap().to_string(),
                    data_type: DataType::Mesh,
                })
            }
            _ => Err(UnimplementedExporterError::new(name, Mesh::NAME).into()),
        }
    }
}
