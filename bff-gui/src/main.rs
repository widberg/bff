#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::platforms::Platform;
use panels::central::view;
use panels::left::resource_list;
use panels::right::resource_info;
use panels::top::menubar;
use three_d::renderer::CpuModel;

mod panels;
mod views;

#[derive(Clone)]
pub enum Artifact {
    Bitmap {
        is_dds: bool, // use enum?
        data: Arc<Vec<u8>>,
    },
    Sound {
        data: Arc<Vec<i16>>,
        sample_rate: u32,
        channels: u16,
    },
    Mesh(Arc<CpuModel>),
    Skin(Arc<CpuModel>),
}

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        })
    });

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        renderer: eframe::Renderer::Glow,
        icon_data: Some(
            eframe::IconData::try_from_png_bytes(include_bytes!("../resources/bff.png")).unwrap(),
        ),
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native("BFF GUI", options, Box::new(|cc| Box::new(Gui::new(cc))))
}

fn setup_custom_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "icons".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/Font Awesome 6 Free-Solid-900.otf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("icons".into()))
        .or_default()
        .push("icons".to_owned());

    ctx.set_fonts(fonts);
}

struct Gui {
    tx: Sender<(BigFile, PathBuf)>,
    rx: Receiver<(BigFile, PathBuf)>,
    bigfile: Option<BigFile>,
    bigfile_path: Option<PathBuf>,
    resource_name: Option<Name>,
    nicknames: HashMap<Name, String>,
    nickname_window_open: bool,
    nickname_editing: (Name, String),
    artifacts: HashMap<Name, Artifact>,
    infos: HashMap<Name, String>,
}

impl Gui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.25);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_custom_font(&cc.egui_ctx);
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx,
            bigfile: None,
            bigfile_path: None,
            resource_name: None,
            nicknames: HashMap::new(),
            nickname_window_open: false,
            nickname_editing: (Name::default(), String::new()),
            artifacts: HashMap::new(),
            infos: HashMap::new(),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok((bf, path)) = self.rx.try_recv() {
            self.bigfile = Some(bf);
            self.bigfile_path = Some(path);
            self.nicknames.clear();
            self.resource_name = None;
            ctx.set_cursor_icon(egui::CursorIcon::Default);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                menubar(
                    ui,
                    frame,
                    ctx,
                    "menubar".into(),
                    &self.bigfile,
                    &self.bigfile_path,
                    &self.resource_name,
                    &mut self.nicknames,
                    &self.artifacts,
                    &self.tx,
                );
                // if let Some((bf, path)) = menubar_response.bigfile_open {
                //     self.bigfile = Some(bf);
                //     self.bigfile_path = Some(path);
                //     self.nicknames.clear();
                //     self.resource_name = None;
                // }

                let resource_list_response = resource_list(
                    ui,
                    format!(
                        "resources-{}",
                        self.bigfile_path
                            .as_ref()
                            .unwrap_or(&PathBuf::default())
                            .display()
                    )
                    .into(),
                    &self.bigfile,
                    &self.nicknames,
                    &self.artifacts,
                    &self.infos,
                    &self.resource_name,
                );
                if let Some(name) = resource_list_response.resource_context_menu {
                    self.nickname_window_open = true;
                    self.nickname_editing.0 = name;
                }
                if let Some(name) = resource_list_response.resource_clicked {
                    self.resource_name = Some(name);
                    if let Some(artifact) = resource_list_response.artifact_created {
                        self.artifacts.insert(name, artifact);
                    }
                    if let Some(info) = resource_list_response.info_created {
                        self.infos.insert(name, info);
                    }
                }

                if let Some(name) = self.resource_name {
                    resource_info(ui, self.infos.get(&name));
                }
                view(ui, "center".into(), &self.resource_name, &self.artifacts);

                if self.nickname_window_open {
                    egui::Window::new("Change resource nickname")
                        .fixed_size(egui::vec2(100.0, 50.0))
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                let output =
                                    egui::TextEdit::singleline(&mut self.nickname_editing.1)
                                        .hint_text("Enter nickname...")
                                        .min_size(ui.available_size())
                                        .show(ui);
                                if (output.response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                                    || ui.button("Change").clicked()
                                {
                                    let filtered_nickname = self.nickname_editing.1.trim();
                                    self.nickname_window_open = false;
                                    if !filtered_nickname.is_empty() {
                                        self.nicknames.insert(
                                            self.nickname_editing.0,
                                            filtered_nickname.to_owned(),
                                        );
                                    } else {
                                        self.nicknames.remove(&self.nickname_editing.0);
                                    }
                                    self.nickname_editing.1 = String::new();
                                }
                            });
                        });
                }
            });

        preview_files_being_dropped(ctx);

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let path = i.raw.dropped_files.get(0).unwrap().path.as_ref().unwrap();
                self.bigfile = Some(load_bigfile(path));
                self.bigfile_path = Some(path.clone());
            }
        });
    }
}

fn load_bigfile(path: &PathBuf) -> BigFile {
    let platform = match path.extension() {
        Some(extension) => extension.try_into().unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(path).unwrap();
    let mut reader = bff::BufReader::new(f);
    BigFile::read_platform(&mut reader, platform).unwrap()
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use std::fmt::Write as _;

    use egui::*;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping BigFile:\n".to_owned();
            if let Some(file) = i.raw.hovered_files.first() {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

trait Export {
    fn export(self) -> Artifact;
}

trait RecursiveExport {
    fn export(self, resources: &HashMap<Name, Class>) -> Artifact;
    fn dependencies(&self) -> Vec<Name> {
        Vec::new()
    }
}

impl Artifact {
    fn save(&self, path: &PathBuf) {
        // let mut file = File::create(path).unwrap();
        match *self {
            Self::Bitmap {
                is_dds: _,
                ref data,
            } => {
                let mut file = File::create(path).unwrap();
                file.write_all(data).unwrap();
            }
            Self::Sound {
                ref data,
                channels,
                sample_rate,
            } => {
                let spec = hound::WavSpec {
                    channels,
                    sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };
                let mut parent_writer = hound::WavWriter::create(path, spec).unwrap();
                let mut sample_writer = parent_writer.get_i16_writer(data.len() as u32);
                for sample in data.iter() {
                    sample_writer.write_sample(*sample);
                }
                sample_writer.flush().unwrap();
                parent_writer.finalize().unwrap();
            }
            Self::Mesh(_) => todo!(),
            Self::Skin(_) => todo!(),
        };
    }
}

impl Export for bff::class::bitmap::v1_291_03_06_pc::BitmapV1_291_03_06PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            is_dds: true,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_381_67_09_pc::BitmapV1_381_67_09PC {
    fn export(self) -> Artifact {
        Artifact::Bitmap {
            is_dds: true,
            data: Arc::new(self.body.data),
        }
    }
}

impl Export for bff::class::bitmap::v1_06_63_02_pc::BitmapV1_06_63_02PC {
    fn export(self) -> Artifact {
        if let Some(dds) = self.body.dds {
            Artifact::Bitmap {
                is_dds: true,
                data: Arc::new(dds),
            }
        } else if let Some(data) = self.body.tex {
            Artifact::Bitmap {
                is_dds: false,
                data: Arc::new(data),
            }
        } else {
            Artifact::Bitmap {
                is_dds: false,
                data: Arc::new(Vec::new()),
            }
        }
    }
}

impl Export for bff::class::sound::v1_291_03_06_pc::SoundV1_291_03_06PC {
    fn export(self) -> Artifact {
        Artifact::Sound {
            data: Arc::new(self.body.data),
            sample_rate: self.body.sample_rate,
            channels: match self.body.flags.stereo().value() {
                1 => 2,
                _ => 1,
            },
        }
    }
}

impl Export for bff::class::sound::v1_381_67_09_pc::SoundV1_381_67_09PC {
    fn export(self) -> Artifact {
        Artifact::Sound {
            data: Arc::new(self.body.data),
            sample_rate: self.link_header.sample_rate,
            channels: match self.link_header.flags.stereo().value() {
                1 => 2,
                _ => 1,
            },
        }
    }
}

trait GenerateMesh {
    fn generate_mesh(&self) -> Vec<three_d::geometry::CpuMesh>;
}

impl GenerateMesh for bff::class::mesh::v1_291_03_06_pc::MeshV1_291_03_06PC {
    fn generate_mesh(&self) -> Vec<three_d::geometry::CpuMesh> {
        use three_d::{Vec2, Vec3, Vec4};
        self.body
            .mesh_buffer
            .vertex_groups
            .iter()
            .map(|group| {
                // println!("{}", mesh.body.mesh_buffer.vertex_buffers.len());
                let (positions, (uvs, (normals, tangents))): (
                    Vec<Vec3>,
                    (Vec<Vec2>, (Vec<Vec3>, Vec<Vec4>)),
                ) = self
                    .body
                    .mesh_buffer
                    .vertex_buffers
                    .iter()
                    .flat_map(|buf| &buf.vertex_structs)
                    .collect::<Vec<&bff::class::mesh::v1_291_03_06_pc::VertexStruct>>()
                    [group.vertex_offset_in_groups as usize
                        ..group.vertex_offset_in_groups as usize + group.vertex_count as usize]
                    .iter()
                    .map(|vs| match vs {
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
                let indices: Vec<u16> = self
                    .body
                    .mesh_buffer
                    .index_buffers
                    .iter()
                    .flat_map(|buf| &buf.tris)
                    .flat_map(|tri| tri.indices)
                    .collect::<Vec<i16>>()[group.index_buffer_offset_in_shorts
                    as usize
                    ..group.index_buffer_offset_in_shorts as usize + group.face_count as usize * 3]
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
            .collect()
    }
}

impl Export for bff::class::mesh::v1_291_03_06_pc::MeshV1_291_03_06PC {
    fn export(self) -> Artifact {
        let tri_meshes = self.generate_mesh();
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
            name: self.name.to_string(),
            geometries: primitives,
            materials: vec![],
        };
        Artifact::Mesh(Arc::new(model))
    }
}

impl RecursiveExport for bff::class::skin::v1_291_03_06_pc::SkinV1_291_03_06PC {
    fn dependencies(&self) -> Vec<Name> {
        let material_crc32s: Vec<Name> = self
            .body
            .skin_sections
            .iter()
            .flat_map(|section| &section.skin_sub_sections.inner)
            .map(|subsection| subsection.material_crc32)
            .collect();
        [self.body.mesh_crc32s.inner.clone(), material_crc32s].concat()
    }
    fn export(self, classes: &HashMap<Name, Class>) -> Artifact {
        let tri_meshes: Vec<three_d_asset::Primitive> = self
            .body
            .mesh_crc32s
            .iter()
            .flat_map(|n| {
                let class = classes.get(n).unwrap();
                match class {
                    Class::Mesh(box_mesh) => match **box_mesh {
                        bff::class::mesh::Mesh::MeshV1_291_03_06PC(ref mesh) => {
                            mesh.generate_mesh()
                        }
                        _ => todo!(),
                    },
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
        let materials = self
            .body
            .skin_sections
            .iter()
            .flat_map(|section| &section.skin_sub_sections.inner)
            .enumerate()
            .map(|(i, subsection)| {
                if let Some(class) = classes.get(&subsection.material_crc32) {
                    match class {
                        Class::Material(box_material) => match **box_material {
                            bff::class::material::Material::MaterialV1_291_03_06PC(
                                ref material,
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
            name: self.name.to_string(),
            geometries: tri_meshes,
            materials,
        };
        Artifact::Skin(Arc::new(model))
    }
}
