// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use bff::bigfile::BigFile;
use bff::names::Name;
use bff::platforms::Platform;
use panels::central::view;
use panels::left::resource_list;
use panels::right::resource_info;
use panels::top::menubar;
use three_d::renderer::CpuModel;

pub mod error;
mod panels;
mod views;

#[derive(Clone)]
pub enum Artifact {
    Bitmap(Vec<u8>),
    Sound {
        data: Arc<Vec<i16>>,
        sample_rate: u32,
        channels: u16,
    },
    Mesh(Arc<CpuModel>),
}

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
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

#[derive(Default)]
struct Gui {
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
        Self {
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
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                let menubar_response = menubar(
                    ui,
                    frame,
                    &self.bigfile,
                    &self.bigfile_path,
                    &self.resource_name,
                    &mut self.nicknames,
                );
                if let Some((bf, path)) = menubar_response.bigfile_open {
                    self.bigfile = Some(bf);
                    self.bigfile_path = Some(path);
                    self.nicknames.clear();
                    self.resource_name = None;
                }

                let resource_list_response = resource_list(
                    ui,
                    &self.bigfile,
                    &self.nicknames,
                    &self.artifacts,
                    &self.infos,
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
            if let Some(file) = i.raw.hovered_files.iter().next() {
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
