#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::HashMap, fs::File, path::PathBuf};

use bff::{
    bigfile::{resource::Resource, BigFile},
    class::{Class, Class::Bitmap},
    names::Name,
    platforms::Platform,
    traits::TryIntoVersionPlatform,
    // versions::Version,
    BufReader,
};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native("BFF GUI", options, Box::new(|_cc| Box::<Gui>::default()))
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> egui::Response {
    ui.add(egui::TextEdit::multiline(&mut text))
}

#[derive(Default)]
struct Gui {
    bigfile: Option<BigFile>,
    resource_name: Option<Name>,
    nicknames: HashMap<Name, String>,
    nickname_window_open: bool,
    nickname_editing: (Name, String),
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.25);
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("top").show_inside(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open BigFile...").clicked() {
                            ui.close_menu();
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.bigfile = Some(load_bigfile(&path));
                            }
                        }
                    });
                });

                egui::SidePanel::left("left")
                    .resizable(true)
                    .width_range(70.0..=ui.available_width() / 2.0)
                    .show_inside(ui, |ui| {
                        // ui.set_width_range(150.0..=200.0);
                        if let Some(bigfile) = &self.bigfile {
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
                                        if ui
                                            .add(
                                                egui::Button::new(format!(
                                                    "{}.{}",
                                                    match self.nicknames.get(&resource.name) {
                                                        Some(nn) => nn.to_owned(),
                                                        None => resource.name.to_string(),
                                                    },
                                                    resource.class_name.to_string()
                                                ))
                                                .wrap(false)
                                                .rounding(0.0)
                                                .min_size(egui::vec2(ui.available_width(), 0.0)),
                                            )
                                            .context_menu(|ui| {
                                                if ui.button("Change nickname").clicked() {
                                                    self.nickname_window_open = true;
                                                    self.nickname_editing.0 = resource.name;
                                                    ui.close_menu();
                                                }
                                            })
                                            .clicked()
                                        {
                                            self.resource_name = Some(resource.name);
                                        }
                                    }
                                },
                            );
                        }
                    });
                egui::SidePanel::right("right")
                    .resizable(true)
                    .width_range(100.0..=ui.available_width() / 2.0)
                    .show_inside(ui, |ui| {
                        if let Some(resource_name) = &self.resource_name {
                            egui::ScrollArea::vertical()
                                .id_source("code_scroll")
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    selectable_text(
                                        ui,
                                        &serde_json::to_string_pretty::<Class>(
                                            &self
                                                .bigfile
                                                .as_ref()
                                                .unwrap()
                                                .objects
                                                .get(resource_name)
                                                .unwrap()
                                                .try_into_version_platform(
                                                    self.bigfile
                                                        .as_ref()
                                                        .unwrap()
                                                        .manifest
                                                        .version
                                                        .clone(),
                                                    self.bigfile
                                                        .as_ref()
                                                        .unwrap()
                                                        .manifest
                                                        .platform
                                                        .clone(),
                                                )
                                                .unwrap(),
                                        )
                                        .unwrap(),
                                    )
                                });
                        }
                    });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if let Some(resource_name) = &self.resource_name {
                        let class: &Class = &self
                            .bigfile
                            .as_ref()
                            .unwrap()
                            .objects
                            .get(resource_name)
                            .unwrap()
                            .try_into_version_platform(
                                self.bigfile.as_ref().unwrap().manifest.version.clone(),
                                self.bigfile.as_ref().unwrap().manifest.platform.clone(),
                            )
                            .unwrap();
                        match class {
                            Bitmap(box_bitmap) => match **box_bitmap {
                                bff::class::bitmap::Bitmap::BitmapV1_291_03_06PC(ref bitmap) => {
                                    ui.add(
                                        egui::Image::new(<(String, std::vec::Vec<u8>) as Into<
                                            egui::ImageSource,
                                        >>::into(
                                            (
                                            format!("bytes://{}.dds", resource_name),
                                            bitmap.body.data.clone(),
                                        )
                                        ))
                                        .texture_options(egui::TextureOptions::NEAREST)
                                        .shrink_to_fit(),
                                    );
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                });
                if self.nickname_window_open {
                    egui::Window::new("Change resource nickname")
                        .fixed_size(egui::vec2(100.0, 50.0))
                        .show(ctx, |ui| {
                            let output = egui::TextEdit::singleline(&mut self.nickname_editing.1)
                                .hint_text("Enter nickname...")
                                .min_size(egui::vec2(100.0, 0.0))
                                .show(ui);
                            if output.response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                let filtered_nickname = self.nickname_editing.1.trim();
                                self.nickname_window_open = false;
                                if filtered_nickname.len() != 0 {
                                    self.nicknames.insert(
                                        self.nickname_editing.0,
                                        filtered_nickname.to_owned(),
                                    );
                                } else {
                                    self.nicknames.remove(&self.nickname_editing.0);
                                }
                            }
                        });
                }
            });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.bigfile = Some(load_bigfile(
                    i.raw.dropped_files.get(0).unwrap().path.as_ref().unwrap(),
                ))
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
    let mut reader = BufReader::new(f);
    BigFile::read_platform(&mut reader, platform).unwrap()
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
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
