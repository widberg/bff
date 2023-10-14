use std::collections::HashMap;

use bff::bigfile::resource::Resource;
use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;
use three_d::renderer::CpuModel;
use three_d_asset::{Vec2, Vec3, Vec4};

use crate::Artifact;

#[derive(Default)]
pub struct ResourceListResponse {
    pub resource_context_menu: Option<Name>,
    pub resource_clicked: Option<Name>,
    pub artifact_created: Option<(Name, Artifact)>,
}

pub fn resource_list(
    ui: &mut egui::Ui,
    bigfile: &Option<BigFile>,
    nicknames: &HashMap<Name, String>,
    artifacts: &HashMap<Name, Artifact>,
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
                let resources: Vec<&Resource> = bigfile.objects.values().collect();
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
                                if artifacts.get(&resource.name).is_none() {
                                    match (*resource)
                                        .try_into_version_platform(version.clone(), platform)
                                    {
                                        Ok(class) => {
                                            if let Some(a) = create_artifact(class, &resource.name)
                                            {
                                                response.artifact_created = Some((resource.name, a))
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

fn create_artifact(class: Class, resource_name: &Name) -> Option<Artifact> {
    match class {
        Class::Bitmap(box_bitmap) => match *box_bitmap {
            bff::class::bitmap::Bitmap::BitmapV1_291_03_06PC(ref bitmap) => {
                Some(Artifact::Bitmap(bitmap.body.data.clone()))
            }
            bff::class::bitmap::Bitmap::BitmapV1_381_67_09PC(ref bitmap) => {
                Some(Artifact::Bitmap(bitmap.body.data.clone()))
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
                data,
                sample_rate,
                channels,
            })
        }
        Class::Mesh(box_mesh) => {
            match *box_mesh {
                bff::class::mesh::Mesh::MeshV1_291_03_06PC(mesh) => {
                    let (positions, (uvs, (normals, tangents))): (Vec<Vec3>, (Vec<Vec2>, (Vec<Vec3>, Vec<Vec4>))) = mesh.body.mesh_buffer.vertex_buffers.iter().flat_map(|buf| &buf.vertex_structs).map(|vs| match vs {
                        bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStruct24 {
                            position, uv, ..
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
                        } => (position, uv, normal, [&tangent[..], &[*tangent_padding]].concat().try_into().unwrap()),
                        bff::class::mesh::v1_291_03_06_pc::VertexStruct::VertexStructUnknown { .. } => {
                            (&[0f32; 3], &[0f32; 2], &[0u8; 3], [0u8; 4])
                        }
                    }).map(|(p, u, n, t)| (Vec3::from(*p), (Vec2::from(*u), (Vec3::from(n.map(|i| (i as f32 - 128.0) / 128.0)), Vec4::from(t.map(|i| (i as f32 - 128.0) / 128.0)))))).unzip();
                    let indices: Vec<u16> = mesh
                        .body
                        .mesh_buffer
                        .index_buffers
                        .iter()
                        .flat_map(|buf| &buf.tris)
                        .flat_map(|tri| tri.indices)
                        .map(|i| u16::try_from(i).unwrap_or(0))
                        .collect();
                    let tri_mesh = three_d::geometry::CpuMesh {
                        positions: three_d::Positions::F32(positions),
                        indices: three_d::Indices::U16(indices),
                        normals: Some(normals),
                        tangents: Some(tangents),
                        uvs: Some(uvs),
                        colors: None,
                    };
                    let triangles = three_d_asset::geometry::Geometry::Triangles(tri_mesh);
                    let primitive = three_d_asset::Primitive {
                        name: "mesh".to_string(),
                        transformation: three_d_asset::Mat4::from_translation([0.0; 3].into()),
                        animations: vec![],
                        geometry: triangles,
                        material_index: None,
                    };
                    let model = CpuModel {
                        name: resource_name.to_string(),
                        geometries: vec![primitive],
                        materials: vec![],
                    };
                    Some(Artifact::Mesh(model))
                }
                _ => None,
            }
        }
        _ => None,
    }
}
