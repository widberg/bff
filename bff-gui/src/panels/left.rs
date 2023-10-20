use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use bff::bigfile::resource::Resource;
use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;
use three_d::renderer::CpuModel;
use three_d_asset::{Vec2, Vec3, Vec4};

use crate::Artifact;

#[derive(Clone, PartialEq)]
enum ListSort {
    Name,
    // NameReverse,
    // Ext,
    // ExtReverse,
}

impl Default for ListSort {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Default, Clone, PartialEq)]
struct ResourceListState {
    sort: ListSort,
    filter: Option<HashMap<Name, bool>>,
    // order: Option<HashMap<usize, Name>>,
}

#[derive(Default)]
pub struct ResourceListResponse {
    pub resource_context_menu: Option<Name>,
    pub resource_clicked: Option<Name>,
    pub artifact_created: Option<Artifact>,
    pub info_created: Option<String>,
}

pub fn resource_list(
    ui: &mut egui::Ui,
    id_source: egui::Id,
    bigfile: &Option<BigFile>,
    nicknames: &HashMap<Name, String>,
    artifacts: &HashMap<Name, Artifact>,
    infos: &HashMap<Name, String>,
) -> ResourceListResponse {
    let mut response = ResourceListResponse::default();
    egui::SidePanel::left("left")
        .resizable(true)
        .width_range(70.0..=ui.available_width() / 2.0)
        .show_inside(ui, |ui| {
            // ui.set_width_range(150.0..=200.0);
            if let Some(bigfile) = bigfile {
                let version = &bigfile.manifest.version;
                let platform = bigfile.manifest.platform;
                let binding = match ui.memory(|mem| {
                    mem.data
                        .get_temp::<Arc<Mutex<ResourceListState>>>(id_source)
                }) {
                    Some(val) => val,
                    None => Arc::new(Mutex::new(ResourceListState::default())),
                };
                let state = binding.lock().unwrap();
                let mut new_state = state.clone();
                let mut class_names = new_state.filter.unwrap_or(
                    bigfile
                        .objects
                        .values()
                        .map(|res| res.class_name)
                        .collect::<HashSet<_>>()
                        .iter()
                        .map(|n| (*n, true))
                        .collect(),
                );
                ui.horizontal(|ui| {
                    // ui.menu_button("Sort", |ui| {
                    //     ui.radio_value(&mut new_state.sort, ListSort::Name, "Name ABC");
                    //     ui.radio_value(&mut new_state.sort, ListSort::NameReverse, "Name XYZ");
                    //     ui.radio_value(&mut new_state.sort, ListSort::Ext, "Extension ABC");
                    //     ui.radio_value(&mut new_state.sort, ListSort::ExtReverse, "Extension XYZ");
                    // });
                    ui.menu_button("Filter", |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            class_names.iter_mut().for_each(|(name, checked)| {
                                ui.checkbox(checked, name.to_string());
                            });
                        });
                    });
                });
                new_state.filter = Some(class_names);
                // let order = new_state.order.clone().unwrap_or_else(|| {
                //     let mut resource_order: Vec<(&Resource, usize)> = bigfile
                //         .objects
                //         .values()
                //         .enumerate()
                //         .map(|(i, r)| (r, i))
                //         .collect();
                //     resource_order.sort_by(|a, b| match new_state.sort {
                //         ListSort::Name => a.0.name.to_string().cmp(&b.0.name.to_string()),
                //         ListSort::NameReverse => b.0.name.to_string().cmp(&a.0.name.to_string()),
                //         ListSort::Ext => {
                //             a.0.class_name.to_string().cmp(&b.0.class_name.to_string())
                //         }
                //         ListSort::ExtReverse => {
                //             b.0.class_name.to_string().cmp(&a.0.class_name.to_string())
                //         }
                //     });
                //     resource_order
                //         .into_iter()
                //         .map(|(r, i)| (i, r.name))
                //         .collect()
                // });
                if new_state != *state {
                    ui.memory_mut(|mem| {
                        mem.data
                            .insert_temp(id_source, Arc::new(Mutex::new(new_state.clone())))
                    });
                }

                // let init_resources: Vec<&Resource> = bigfile.objects.values().collect();
                let resources: Vec<&Resource> = bigfile
                    .objects
                    .values()
                    .filter(|res| {
                        *state
                            .filter
                            .as_ref()
                            .unwrap_or(&HashMap::default())
                            .get(&res.class_name)
                            .unwrap_or(&true)
                    })
                    .collect();

                let row_height = ui.spacing().interact_size.y;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    resources.len(),
                    |ui, row_range| {
                        ui.set_min_width(ui.available_width());
                        for row in row_range {
                            let resource = resources.get(row).unwrap();
                            let nickname = nicknames.get(&resource.name);
                            let temp_btn = ui
                                .add(
                                    egui::Button::new(format!(
                                        "{}.{}",
                                        match nickname {
                                            Some(nn) => nn.to_owned(),
                                            None => resource.name.to_string(),
                                        },
                                        resource.class_name
                                    ))
                                    .wrap(false)
                                    .rounding(0.0)
                                    .min_size(egui::vec2(ui.available_width(), 0.0)),
                                )
                                .context_menu(|ui| {
                                    if ui.button("Change nickname").clicked() {
                                        // self.nickname_window_open = true;
                                        // self.nickname_editing.0 = resource.name;
                                        response.resource_context_menu = Some(resource.name);
                                        ui.close_menu();
                                    }
                                });
                            let btn = if nickname.is_some() {
                                temp_btn.on_hover_ui_at_pointer(|ui| {
                                    ui.label(resource.name.to_string());
                                })
                            } else {
                                temp_btn
                            };
                            if btn.clicked() {
                                response.resource_clicked = Some(resource.name);
                                if artifacts.get(&resource.name).is_none()
                                    || infos.get(&resource.name).is_none()
                                {
                                    match (*resource)
                                        .try_into_version_platform(version.clone(), platform)
                                    {
                                        Ok(class) => {
                                            response.info_created = Some(
                                                serde_json::to_string_pretty::<Class>(&class)
                                                    .unwrap(),
                                            );
                                            if let Some(a) =
                                                create_artifact(bigfile, class, &resource.name)
                                            {
                                                response.artifact_created = Some(a);
                                            }
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                        }
                                    }
                                }
                                // self.resource_name = Some(resource.name);
                            }
                        }
                    },
                );
            }
        });
    response
}

fn create_artifact(bigfile: &BigFile, class: Class, resource_name: &Name) -> Option<Artifact> {
    match class {
        Class::Bitmap(box_bitmap) => match *box_bitmap {
            bff::class::bitmap::Bitmap::BitmapV1_291_03_06PC(bitmap) => {
                Some(Artifact::Bitmap(bitmap.body.data))
            }
            bff::class::bitmap::Bitmap::BitmapV1_381_67_09PC(bitmap) => {
                Some(Artifact::Bitmap(bitmap.body.data))
            }
            _ => None,
        },
        Class::Sound(box_sound) => {
            let (data, sample_rate, channels) = match *box_sound {
                bff::class::sound::Sound::SoundV1_291_03_06PC(sound) => {
                    // let points = sound.body.data.iter().enumerate().map(|(i, s)| eframe::epaint::Pos2{x: ((i as f32 * ui.available_width()) / sound.body.data.len() as f32), y: (s / 200 + 200).into()}).collect();
                    // let shape = eframe::epaint::PathShape::line(points, eframe::epaint::Stroke::new(1.0, eframe::epaint::Color32::WHITE));
                    // ui.painter().add(shape);
                    (
                        sound.body.data,
                        sound.body.sample_rate,
                        match sound.body.flags.stereo().value() {
                            1 => 2,
                            _ => 1,
                        },
                    )
                }
                bff::class::sound::Sound::SoundV1_381_67_09PC(sound) => (
                    sound.body.data,
                    sound.link_header.sample_rate,
                    match sound.link_header.flags.stereo().value() {
                        1 => 2,
                        _ => 1,
                    },
                ),
            };
            Some(Artifact::Sound {
                data: Arc::new(data),
                sample_rate,
                channels,
            })
        }
        Class::Mesh(box_mesh) => match get_mesh(*box_mesh) {
            Some(tri_meshes) => {
                let primitives = tri_meshes
                    .into_iter()
                    .map(|m| {
                        let triangles = three_d_asset::geometry::Geometry::Triangles(m);
                        three_d_asset::Primitive {
                            name: "mesh".to_string(),
                            transformation: three_d_asset::Mat4::from_translation([0.0; 3].into()),
                            animations: vec![],
                            geometry: triangles,
                            material_index: None,
                        }
                    })
                    .collect();
                let model = CpuModel {
                    name: resource_name.to_string(),
                    geometries: primitives,
                    materials: vec![],
                };
                Some(Artifact::Mesh(Arc::new(model)))
            }
            None => None,
        },
        Class::Skin(box_skin) => match *box_skin {
            bff::class::skin::Skin::SkinV1_291_03_06PC(skin) => {
                let tri_meshes: Vec<three_d_asset::Primitive> = skin
                    .body
                    .mesh_crc32s
                    .iter()
                    .flat_map(|n| {
                        let res = bigfile.objects.get(n).unwrap();
                        let class: Class = res
                            .try_into_version_platform(
                                bigfile.manifest.version.clone(),
                                bigfile.manifest.platform,
                            )
                            .unwrap();
                        match class {
                            Class::Mesh(box_mesh) => get_mesh(*box_mesh).unwrap(),
                            _ => panic!("not a mesh?"),
                        }
                    })
                    .enumerate()
                    .map(|(i, mesh)| {
                        let triangles = three_d_asset::geometry::Geometry::Triangles(mesh);
                        three_d_asset::Primitive {
                            name: format!("skin-part{}", i),
                            transformation: three_d_asset::Mat4::from_translation([0.0; 3].into()),
                            animations: vec![],
                            geometry: triangles,
                            material_index: Some(i),
                        }
                    })
                    .collect();
                let materials = skin
                    .body
                    .skin_sections
                    .iter()
                    .flat_map(|section| &section.skin_sub_sections.inner)
                    .enumerate()
                    .map(|(i, subsection)| {
                        if let Some(res) = bigfile.objects.get(&subsection.material_crc32) {
                            let class: Class = res
                                .try_into_version_platform(
                                    bigfile.manifest.version.clone(),
                                    bigfile.manifest.platform,
                                )
                                .unwrap();
                            match class {
                                Class::Material(box_material) => match *box_material {
                                    bff::class::material::Material::MaterialV1_291_03_06PC(
                                        material,
                                    ) => three_d::renderer::material::CpuMaterial {
                                        name: format!("{}-mat{}", subsection.material_crc32, i),
                                        albedo: material.body.diffuse_color.into(),
                                        emissive: material.body.emissive_color.into(),
                                        ..Default::default()
                                    },
                                    _ => todo!(),
                                },
                                _ => panic!("not a material?"),
                            }
                        } else {
                            three_d::renderer::material::CpuMaterial {
                                name: format!("{}-mat", subsection.material_crc32),
                                ..Default::default()
                            }
                        }
                    })
                    .collect();

                let model = three_d::renderer::object::CpuModel {
                    name: resource_name.to_string(),
                    geometries: tri_meshes,
                    materials,
                };
                Some(Artifact::Skin(Arc::new(model)))
            }
            _ => None,
        },
        _ => None,
    }
}

fn get_mesh(mesh: bff::class::mesh::Mesh) -> Option<Vec<three_d_asset::TriMesh>> {
    match mesh {
        bff::class::mesh::Mesh::MeshV1_291_03_06PC(mesh) => {
            let tri_meshes = mesh
                .body
                .mesh_buffer
                .vertex_groups
                .iter()
                .map(|group| {
                    // println!("{}", mesh.body.mesh_buffer.vertex_buffers.len());
                    let (positions, (uvs, (normals, tangents))): (
                        Vec<Vec3>,
                        (Vec<Vec2>, (Vec<Vec3>, Vec<Vec4>)),
                    ) = mesh
                        .body
                        .mesh_buffer
                        .vertex_buffers
                        .iter()
                        .flat_map(|buf| &buf.vertex_structs)
                        .collect::<Vec<&bff::class::mesh::v1_291_03_06_pc::VertexStruct>>()
                        [group.vertex_offset_in_groups as usize
                            ..group.vertex_offset_in_groups as usize + group.vertex_count as usize]
                        .iter()
                        .map(|vs| {
                            match vs {
                    bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStruct24 {
                        position,
                        uv,
                        ..
                    } => (position, uv, &[0u8; 3], [0u8; 4]),
                    bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStruct36 {
                        position,
                        uv,
                        normal,
                        tangent,
                        tangent_padding,
                        ..
                    }
                    | bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStruct48 {
                        position,
                        uv,
                        normal,
                        tangent,
                        tangent_padding,
                        ..
                    }
                    | bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStruct60 {
                        position,
                        uv,
                        normal,
                        tangent,
                        tangent_padding,
                        ..
                    } => (
                        position,
                        uv,
                        normal,
                        [&tangent[..], &[*tangent_padding]]
                            .concat()
                            .try_into()
                            .unwrap(),
                    ),
                    bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStructUnknown {
                        ..
                    } => (&[0f32; 3], &[0f32; 2], &[0u8; 3], [0u8; 4]),
                }
                        })
                        .map(|(p, u, n, t)| {
                            (
                                Vec3::from(*p),
                                (
                                    Vec2::from(*u),
                                    (
                                        {
                                            let mut norm = n.map(|i| (i as f32 - 128.0) / 128.0);
                                            norm[2] *= -1.0;
                                            Vec3::from(norm)
                                        },
                                        Vec4::from(t.map(|i| (i as f32 - 128.0) / 128.0)),
                                    ),
                                ),
                            )
                        })
                        .unzip();
                    let indices: Vec<u16> = mesh
                        .body
                        .mesh_buffer
                        .index_buffers
                        .iter()
                        .flat_map(|buf| &buf.tris)
                        .flat_map(|tri| tri.indices)
                        .collect::<Vec<i16>>()[group
                        .index_buffer_offset_in_shorts
                        as usize
                        ..group.index_buffer_offset_in_shorts as usize
                            + group.face_count as usize * 3]
                        .iter()
                        .map(|i| u16::try_from(*i).unwrap_or(0) - group.vertex_offset_in_groups)
                        .collect();
                    three_d::geometry::CpuMesh {
                        positions: three_d::Positions::F32(positions),
                        indices: three_d::Indices::U16(indices),
                        normals: Some(normals),
                        tangents: Some(tangents),
                        uvs: Some(uvs),
                        colors: None,
                    }
                })
                .collect();
            Some(tri_meshes)
        }
        _ => None,
    }
}
